import "./styles/components.scss";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useStoreActions, useStoreState } from "./store/hook.store";
export enum MapType {
    MapFrom,
    MapTo
}

interface ButtonShortcutProps {
    id: number;
    keybind: string;
    mapType: MapType;
}

export const ButtonShortcut = ({ id, keybind, mapType }: ButtonShortcutProps) => {
    const [keyBind, setKeyBind] = useState<string>(keybind);

    const { dbInstance, dbCopyInstance, dbHasChange, dbIsValid } = useStoreState((store) => {
        return store.dbModel;
    });
    const { loadDbInstance, validateDb, updateValueCopyDb, updateKeyCopyDb, setDbHasChange, setIsRecording } = useStoreActions((actions) => actions.dbModel)
    async function record() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        setIsRecording(true);

        let prev_keybind = keyBind.split(" + ");

        let rt:string = await invoke("record");
        console.log(rt);
        if (rt == "" || rt == undefined || rt == null) {
            setIsRecording(false);
            console.log("No keybind recorded");
            return;
        }
        let parsed_string = JSON.parse(rt);
        let obj: string[] = JSON.parse(parsed_string);
        console.log(obj);
        // Update the copy db

        if (mapType == MapType.MapFrom) {
            updateKeyCopyDb({ index: id, newKey: obj });
            setDbHasChange();
            validateDb();
        }
        else {
            console.log(dbCopyInstance.length);
            updateValueCopyDb({ index: id, value: obj });
            setDbHasChange();
            validateDb();
            console.log(dbIsValid);
        }
        setKeyBind(obj.join(" + "));
        setIsRecording(false);
    }
    return (
        <div className="button-shortcut" onClick={() => record()}>
            <p>{keyBind}</p>
        </div>
    )
}

