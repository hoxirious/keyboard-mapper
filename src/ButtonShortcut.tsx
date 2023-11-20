import "./styles/components.scss";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useStoreActions, useStoreState } from "./store/hook.store";
export enum MapType {
    MapFrom,
    MapTo
}

interface ButtonShortcutProps {
    keybind: string;
    mapType: MapType;
}

export const ButtonShortcut = ({ keybind, mapType }: ButtonShortcutProps) => {
    const [keyBind, setKeyBind] = useState<string>(keybind);

    const { dbInstance, dbCopyInstance, dbHasChange, dbIsValid } = useStoreState((store) => {
        return store.dbModel;
    });
    const { loadDbInstance, validateDb, updateValueCopyDb, updateKeyCopyDb, setDbHasChange } = useStoreActions((actions) => actions.dbModel)
    async function record() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        let prev_keybind = keyBind.split(" + ");

        let parsed_string = JSON.parse(await invoke("record"));
        let obj: string[] = JSON.parse(parsed_string);
        console.log(obj);
        // Update the copy db

        if (mapType == MapType.MapFrom) {
            updateKeyCopyDb({ oldKey: prev_keybind, newKey: obj });
            setDbHasChange();
            validateDb();
        }
        else {
            console.log(dbCopyInstance.length);
            // value cannot find the key
            updateValueCopyDb({ key: prev_keybind, value: obj });
            setDbHasChange();
            validateDb();
            console.log(dbIsValid);
        }
        setKeyBind(obj.join(" + "));
    }
    return (
        <div className="button-shortcut" onClick={() => record()}>
            <p>{keyBind}</p>
        </div>
    )
}

