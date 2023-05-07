# My Project Journal

- [x] Key mapping
- [x] Listen for combination
- [ ] Error handling
- [ ] Native app

---

- `listen` to listen to keyboard events on a port (like socket)
- `grab` to grab keyboard event globably
- `serialize` to serialize event data

---
**Couldn't run grab function**. Encounter this error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
Resolved by add `input` and `plugdev` into current user's group. Reboot after change

---
**Question**: how to record combination keybind on global. We need some kind of keypress/keyrelease controller
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
**Create a mapper list**. Record combination will reflect on that list to decide whether mapping or not.
Use lazy_static for static global variables: <https://docs.rs/lazy_static/1.4.0/lazy_static/>

---
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
**New problem:**

- Simulations are listened by the `listen()` method which causes infinite loop.
- Need to refactor the logic **again** so that we ensure only physically pressed/released key are listened and emitted event.
