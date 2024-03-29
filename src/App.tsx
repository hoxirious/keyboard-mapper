import { ReactNode, useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { ButtonShortcut } from "./ButtonShortcut";
import { MapperShortcut } from "./MapperShortcut";
import { useStoreActions, useStoreState } from "./store/hook.store";
import db from "../src-tauri/maplist.json"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrashCan } from "@fortawesome/free-regular-svg-icons";
import "./styles/components.scss"
import { appWindow } from '@tauri-apps/api/window'
import { a } from "@tauri-apps/api/tauri-605fa63e";
export type DbInstanceType = {
    key: string[];
    value: ({ KeyPress: string; KeyRelease?: undefined; }
        | { KeyRelease: string; KeyPress?: undefined; })[];
}[]

function App() {

    const { dbInstance, isRecording, dbCopyInstance, dbHasChange, dbIsValid, mapStarted } = useStoreState((store) => {
        return store.dbModel;
    });
    const { loadDbInstance, validateDb, createKeybind,setMapStarted } = useStoreActions((actions) => actions.dbModel);

    useEffect(() => {
        const dbInstance: DbInstanceType = db;
        loadDbInstance(dbInstance);
    }, [db])

    useEffect(() => {
        validateDb();
    }, [dbCopyInstance])

    const parseMapFrom = (key: string[]) => {
        return key.join(" + ");
    }

    const parseMapTo = (value: ({ KeyPress: string; KeyRelease?: undefined; } | { KeyRelease: string; KeyPress?: undefined; })[]) => {
        let rt = value.map((item) => {
            if (item.KeyPress) {
                return item.KeyPress;
            }
        })
        return rt.join(" + ").slice(0, -6);
    }

    function addNewHolder() {
        createKeybind();
    }

    async function saveChanges(): Promise<void> {
        if (dbIsValid) {
            console.log("Saving changes...");
            await invoke("save_db", { db: JSON.stringify(dbCopyInstance) });

        } else {
            // alert("Invalid database!");
            console.log("Invalid database!");
        }
    }

    async function startMapper(): Promise<void> {
        if (dbIsValid) {
            setMapStarted(true);
            await invoke("start_mapper");
            await invoke('hide_window');
        }
    }

    return (
        <div className="container">
            {isRecording && (
                <div className="recording-block">
                    <div>
                        <p>Recording...Press Esc to exit</p>
                    </div>
                </div>
            )}
            <h1 className="text-3xl font-bold underline text-black">
                Keyboard Mapper
            </h1>
            {
                dbCopyInstance.map((item, index) => {
                    return (
                        <div className="row">
                            <MapperShortcut key={index} keybind_id={index} mapfrom={parseMapFrom(item.key)} mapto={parseMapTo(item.value)} />
                        </div>
                    )
                })}
            <div className="row">
                <button onClick={() => addNewHolder()}>New Keybind</button>
                {dbHasChange && (
                    <button id="save-button" onClick={() => saveChanges()}>Save Changes</button>
                )}
                {mapStarted && (
                    <button id="start-mapper" disabled>Start Mapper</button>
                )}
                {!mapStarted && (
                    <button id="start-mapper" onClick={() => startMapper()}>Start Mapper</button>
                )
                }
            </div>
        </div>
    );
}

export default App;
