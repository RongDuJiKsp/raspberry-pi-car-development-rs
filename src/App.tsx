import {useEffect, useState} from 'react';
import reactLogo from './assets/react.svg';
import {invoke} from '@tauri-apps/api/core';
import './App.css';

const comboDelta: number = 0.1;
const asCombo = (x: string): number => {
    const v = Number(x);
    const [min, max] = [0.2, 1];
    if (v < min) return min;
    if (v > max) return max;
    else return v;
}
const speedDelta: number = 2;
const asSpeed = (x: string): number => {
    const v = Number(x);
    const [min, max] = [20, 50];
    if (v < min) return min;
    if (v > max) return max;
    else return v;
}
export default function App() {
    const [addr, setaddr] = useState('');
    const [combo, setCombo] = useState(asCombo("-1"));
    const [speed, setSpeed] = useState(asSpeed("-1"));

    async function exec(cmd: string, addr: string) {
        await invoke(cmd, {addr});
    }

    const stop = () => {
        exec('stop', addr);
    };
    useEffect(() => {
        const down = (event: KeyboardEvent) => {
            switch (event.key.toUpperCase()) {
                case 'W':
                    exec('go', addr);
                    break;
                case 'A':
                    exec('left', addr);
                    break;
                case 'S':
                    exec('back', addr);
                    break;
                case 'D':
                    exec('right', addr);
                    break;
                default:
                    return;
            }
            event.preventDefault();
        };
        const up = (_event: KeyboardEvent) => {
            exec('stop', addr);
        };
        window.addEventListener('keydown', down);
        window.addEventListener('keyup', up);
        return () => {
            window.removeEventListener('keydown', down);
            window.removeEventListener('keyup', up);
        };
    }, [addr]);
    return (
        <main className="container">
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
            <div className="main-conroller">
                <input onChange={(e) => setaddr(e.target.value)} placeholder={"小车地址，如127.0.0.1:37037"}></input>
                <button className="conn-btn" onClick={() => exec('connto', addr)}>
                    Conn
                </button>
            </div>
            <div className="main-conroller">
                <div className="controller-line">
                    <button className="speed-btn"
                            onClick={() => setCombo(r => asCombo(String(r - comboDelta)))}>
                        -
                    </button>
                    <input className={"speed-input"} placeholder={"转向速度差"} type={"number"} value={combo.toFixed(1)}
                           onChange={(e) => setCombo(asCombo(e.target.value))}/>
                    <button className="speed-btn"
                            onClick={() => setCombo(r => asCombo(String(r + comboDelta)))}>
                        +
                    </button>
                </div>
                <div className="controller-line">
                    <button className="speed-btn"
                            onClick={() => setSpeed(r => asSpeed(String(r - speedDelta)))}>
                        -
                    </button>
                    <input className={"speed-input"} placeholder={"速度"} type={"number"} value={speed.toFixed(0)}
                           onChange={(e) => setSpeed(asSpeed(e.target.value))}/>
                    <button className="speed-btn"
                            onClick={() => setSpeed(r => asSpeed(String(r + speedDelta)))}>
                        +
                    </button>
                </div>
            </div>
            <div className="main-conroller">
                <div className="controller-line">
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('go', addr)}
                        onMouseUp={stop}
                    >
                        GoLeft
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('go', addr)}
                        onMouseUp={stop}
                    >
                        Go
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('go', addr)}
                        onMouseUp={stop}
                    >
                        GoRight
                    </button>
                </div>
                <div className="controller-line">
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('left', addr)}
                        onMouseUp={stop}
                    >
                        Left
                    </button>
                    <button className="controller-btn" onClick={() => exec('stop', addr)}>
                        Stop
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('right', addr)}
                        onMouseUp={stop}
                    >
                        Right
                    </button>
                </div>
                <div className="controller-line">
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('go', addr)}
                        onMouseUp={stop}
                    >
                        BackLeft
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('go', addr)}
                        onMouseUp={stop}
                    >
                        Back
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => exec('go', addr)}
                        onMouseUp={stop}
                    >
                        BackRight
                    </button>
                </div>
            </div>
        </main>
    );
}


