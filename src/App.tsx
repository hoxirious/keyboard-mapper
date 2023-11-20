import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { ButtonShortcut } from "./ButtonShortcut";
import { MapperShortcut } from "./MapperShortcut";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet"));
    }
    async function meet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("meet"));
    }
    async function record() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

        let parsed_string = JSON.parse(await invoke("record"));
        setGreetMsg(parsed_string);
    }


    return (
        <div className="container">

            <h1 className="text-3xl font-bold underline text-black">
                Keyboard Mapper
            </h1>

            <div className="row">
                <MapperShortcut />
            </div>
            <div className="row">
                <form
                    onSubmit={(e) => {
                        e.preventDefault();
                        greet();
                    }}
                >
                    <input
                        id="greet-input"
                        onChange={(e) => setName(e.currentTarget.value)}
                        placeholder="Enter a name..."
                    />
                    <button type="submit">Greet</button>
                </form>

                <div className="row">
                    <form
                        onSubmit={(e) => {
                            e.preventDefault();
                            record();
                        }}
                    >
                        <input
                            id="meet-input"
                            onChange={(e) => setName(e.currentTarget.value)}
                            placeholder="Enter a name..."
                        />
                        <button type="submit">Meet</button>
                    </form>
                </div>
            </div>
            <p>{greetMsg}</p>
        </div>
    );
}

export default App;
