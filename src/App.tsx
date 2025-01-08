import {useState} from "react";
import reactLogo from "./assets/react.svg";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [addr, setaddr] = useState("");

    async function go(addr: string) {
        await invoke("go", {addr})
    }

    async function stop(addr: string) {
        await invoke("stop", {addr})
    }

    async function conn(addr: string) {
        await invoke("connto", {addr})
    }

    return (
        <main className="container">
            <h1>Welcome to Tauri + React</h1>
            <div className="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img src="/vite.svg" className="logo vite" alt="Vite logo"/>
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img src="/tauri.svg" className="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://reactjs.org" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo"/>
                </a>
            </div>
            <p>Click on the Tauri, Vite, and React logos to learn more.</p>
            <button onClick={() => go(addr)}>Go</button>
            <button onClick={() => stop(addr)}>Stop</button>
            <button onClick={() => conn(addr)}>Conn</button>
            <input onChange={(e) => setaddr(e.target.value)}></input>
        </main>
    );
}

export default App;
