import { useCallback, useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { metaKeys } from "./utils/keyboard";

interface IKeyEvent {
    key: string;
    pressed: boolean;
}

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");
    const [recordedKeys, setRecordedKeys] = useState<IKeyEvent[]>([]);
    const [keyPressed, setKeyPressed] = useState<string[]>([]);
    // const [recordedCombination, setRecordedCombination] = useState<string[]>([]);

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet", { name }));
    }

    const appendKey = (callback: React.Dispatch<React.SetStateAction<IKeyEvent[]>>, obj: IKeyEvent[], event: KeyboardEvent, isPressed: boolean) => {
        let normalKey: IKeyEvent = {
            key: "Key" + event.key.toUpperCase(),
            pressed: isPressed
        }

        let functKey: IKeyEvent = {
            key: event.key,
            pressed: isPressed
        }

        if (!obj.includes(normalKey))
            callback(prev => ([...prev, normalKey]));
        else if (!obj.includes(functKey))
            callback(prev => ([...prev, functKey]));;
    }

    const emitEvent = () => {
        console.log(keyPressed);
    }

    const appendKeyPressed = useCallback((event: KeyboardEvent) => {
        if (event.key === "Enter") {
            emitEvent();
            setKeyPressed([]);
            return;
        }
        else {

            if(!keyPressed.includes(metaKeys.super)) {
                setKeyPressed(prev => ([...prev, metaKeys.super]));
            }
            else if(event.metaKey && !keyPressed.includes(metaKeys.meta)) {
                setKeyPressed(prev => ([...prev, metaKeys.meta]));
            }
            else if(event.ctrlKey && !keyPressed.includes(metaKeys.control)) {
                setKeyPressed(prev => ([...prev, metaKeys.control]));
            }
            else if (event.altKey && !keyPressed.includes(metaKeys.alt)) {
                setKeyPressed(prev => ([...prev, metaKeys.alt]));
            }
            else if (event.shiftKey && !keyPressed.includes(metaKeys.shift)) {
                setKeyPressed(prev => ([...prev, metaKeys.shift]));
            }
            else if (!keyPressed.includes(event.key)) {
                setKeyPressed(prev => ([...prev, "Key" + event.key.toUpperCase()]));
            }


        }
    }, [keyPressed])

    useEffect(() => {

        document.addEventListener("keydown", appendKeyPressed, false);
        // document.addEventListener("keyup", recordCombination, false);

        return () => {
            document.removeEventListener("keydown", appendKeyPressed, false);
            // document.removeEventListener("keyup", recordCombination, false);
        }
    }, [appendKeyPressed]);


    return (
        <div className="container">
            <h1>Welcome to Tauri!</h1>

            <div className="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img src="/vite.svg" className="logo vite" alt="Vite logo" />
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://reactjs.org" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo" />
                </a>
            </div>

            <p>Click on the Tauri, Vite, and React logos to learn more.</p>

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
            </div>
            <p>{greetMsg}</p>
        </div>
    );
}

export default App;
