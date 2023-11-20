import "./styles/components.scss";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

interface ButtonShortcutProps {
    keybind: string;
}

export const ButtonShortcut = ({ keybind }: ButtonShortcutProps) => {
    const [keyBind, setKeyBind] = useState<string>(keybind);
    async function record() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        let parsed_string = JSON.parse(await invoke("record"));
        let obj: string[] = JSON.parse(parsed_string);
        console.log(obj);

        setKeyBind(obj.join(" + "));
    }
    return (
        <div className="button-shortcut" onClick={() => record()}>
            <p>{keyBind}</p>
        </div>
    )
}

