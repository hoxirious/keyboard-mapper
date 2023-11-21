# My Project Journal
[![DEMO](https://img.youtube.com/vi/b2BASlCg71c/hqdefault.jpg)](https://youtu.be/b2BASlCg71c)

---

## Chapter 1: PAIN - TO RELEASE THE WEAKNESS

---

- [x] Key mapping
- [x] Listen for combination
- [ ] Native app

---

- `listen` to listen to keyboard events on a port (like socket)
- `grab` to grab keyboard event globably
- `serialize` to serialize event data

---

### Couldn't run grab function

Encounter this error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
Resolved by add `input` and `plugdev` into current user's group. Reboot after change

- deps: build-essential, pkg-config, libevdev-dev

```bash
sudo apt install -y libgtk-3-dev clang libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

### Requirement

Thanks @jersou for the setup solution: <https://github.com/jersou/mouse-actions>
To use the main feature "grab event", you need to have the read&write permission
on `/dev/input/event*`. Check the group of `/dev/input/event*` files :

```bash
ls -al /dev/input/event*
# > crw-rw---- 1 root input /dev/input/event5
#                     ^^^^^
```

You need to add the current user to this group, usually `input` or `plugdev` :

```bash
sudo usermod -a -G plugdev $USER
# or
sudo usermod -a -G input $USER
```

Furthermore, you must have the read&write permission on `/dev/uinput`, you can
check with:

```bash
getfacl /dev/uinput
# ...
# user:<the current user>:rw-
# ...
```

If this permission is not available on the user, to add it
temporary : `sudo setfacl -m u:$USER:rw /dev/uinput` or persistent :

```bash
sudo tee /etc/udev/rules.d/80-mouse-actions.rules <<<'KERNEL=="uinput", SUBSYSTEM=="misc", TAG+="uaccess", OPTIONS+="static_node=uinput"'
```

You need to restart your desktop session to apply these changes.

To check the user groups and the ACL after the session restart or the reboot:

```bash
$ groups
... input ...
$ getfacl /dev/uinput
# ...
# user:<the current user>:rw-
# ...
```

---

### Question

how to record combination keybind on global. We need some kind of keypress/keyrelease controller
Enum `EventType` has `KeyPress` and `KeyRelease`. What can we do with it?
Create vec[] of `eventData` that will be appended based on EventType:

- `EventType::KeyPress`: append event into vec
- `EventType::KeyRelease`: End of recording -> pop the according event
At the end of the release state, if vec[] is empty then emit the action regards to the recorded combination

- **Restriction**:
  - Only record a key that is not being pressed or a key that is already released.
  - Do not emit action while a key is pressed.
  - Only put on listening mode if a special key (control, shift, cmd) is pressed. Otherwise, ignore

**Or let's reconsider:**
We just need to map Ctrl to Cmd. This is a hacky way but not the right way. Map one key will possible break different combination. eg: _Control-Command-F: Use the app in full screen, if supported by the app._ will become _Command-Command-F_

---

### Create a mapper list

Record combination will reflect on that list to decide whether mapping or not.
Use lazy_static for static global variables: <https://docs.rs/lazy_static/1.4.0/lazy_static/>

---

### Rework logic

**Need to rework how app record event:**

- When a key is pressed: two types of key
      1. Special keys
      2. Non-special keys
      - Special key means combination is coming -> we need to record it to map the combination. **Only record one time**
      - Non-special keys means:
          + if no special keys in front, it is single action so no record needed -> emit
          + otherwise -> record -> emit -> reset

**Result of this new logic:** Mapped and unmapped hotkeys work well except for ones that are not managed under applications but the operation system, eg: Meta key, Alt tab,...
**Reason**: I did not dig too deep into the problem but (i could be wrong) my guess was the `simulate()` of the [rdev](https://github.com/Narsil/rdev) couldn't simulate "window-level" events. I visit different repos like `rustdust` and they use the [forked repo](https://github.com/fufesou/rdev). After the trial with new library, the above issue was fixed. But apparently the library has a lot of refactors, so most of my previous work needs to be refactored as well.

- For example: `grab` feature is reworked. From my best understanding, this feature is now more leaning toward sockets use case. We need to have a socket open to have grab loop opens.

> Event flow from `rustdust`: `on_connected -> start_grab_loop -> start_grab_listen -> on key press/release: process_event`

**Incompatible Problem**: `start_grab_loop` does not loop itself like `grab` used to be. So the listener ends right away.
**Resolution**: use `listen()` instead. Apparently, this listener is now work globally. Could be a replacement for the old `grab`.

---

### New problem:

- Simulations are listened by the `listen()` method which causes infinite loop.
- Need to refactor the logic **again** so that we ensure only physically pressed/released key are listened and emitted event.

- Refactor logic:

> - Key Press:
>   - if special key -> set special key hashmap to true -> return
>   - else -> process_event
> - Key Release:
>   - if special key -> set special key hashmap to false
>   - else -> process_event
> - Process Event:
>   - Handle both Press/Release events
>   - One record vector - push when press, pop when release. Then gather combination and emit.
>   - Release does not need to get special key.

---

### BREAKING NEWS

- Circle back to Narsil's rdev. **AND I HAD FOUND THE ROOT OF THE PAIN!!!!!**
- grab's callback has `Option<Event>` as a return type. Which is an option to decide passing the event to the OS or not. Silly me! I almost gave up until I decided to look back where I started...

This means **Phase 1** has closed. Thank you for joining me on this **First Chapter of the Rust journey**

---

## Chapter 2: NATIVE APP - PROMISING LAND

---

### Intial test and thought

- There will be a frontend built on local. Send API request to backend
- Now let's imagine the workflow:
  - UI to setup key mapping: start mapping, frontend recaps the keyboard events (from and to), send to BE, BE do some schema mapping (event type in js is different from event type rdev), persist it
  - UI to start/stop: send request to BE, BE start/stop `grab`.
  - UI to displays keymap.

### After trial and error

- I have tried multiple methods:
1. Grabbing event from client (like the initial thought), however browser listener is limitted in this feature, so not all key pressed is recorded.
2. Grabbing from backend:
    - Using Tauri State for globally manage state between custom commands, trying to keep a global channel to end the the grab loop.
        - Failed, because the grab loop has blocked the receiver listener.
    - Finally, I decided to dig deep into `rdev` library, and found `GrabStatus::Stop` flag that primarily used for break out of grab loop
    Yet has not been called anywhere. So I tweaked the library and voila!.

### Planning for next step

#### Workflow:

1. Create `record_keybind` that record user's customization keybinds. This includes the method to stop recording.

2. Create functions to write back to json and reload static json

3. Create `start_grab` with the exisiting logic

4. Frontend thingy.


#### Todo
1. Save keybind's mapto to the mapfrom. Need to link up between key and value.
2. Allow add more keybind
3. Persist to json file and reload
4. Start grabbing.
