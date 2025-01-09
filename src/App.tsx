import {useEffect, useState} from 'react';
import reactLogo from './assets/react.svg';
import {invoke, InvokeArgs} from '@tauri-apps/api/core';
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
const exec = (cmd: string, args?: InvokeArgs): void => {
    invoke(cmd, args).catch(console.error);
}
export default function App() {
    const [addr, setAddr] = useState('');
    const [combo, setCombo] = useState(asCombo("-1"));
    const [speed, setSpeed] = useState(asSpeed("-1"));

    const run = (cmd: string): void => {
        exec(cmd, {addr, speed});
    }

    const stop = () => {
        exec("stop");
    };
    const turn = (cmd: string): void => {
        exec(cmd, {addr, speed, combo});
    }
    useEffect(() => {
        const down = (event: KeyboardEvent) => {
            switch (event.key.toUpperCase()) {
                case 'W':
                    run('go');
                    break;
                case 'A':
                    run('left');
                    break;
                case 'S':
                    run('back');
                    break;
                case 'D':
                    run('right');
                    break;
                default:
                    return;
            }
            event.preventDefault();
        };
        const up = (_event: KeyboardEvent) => {
            run('stop');
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
                <input onChange={(e) => setAddr(e.target.value)} placeholder={"小车地址，如127.0.0.1:37037"}></input>
                <button className="conn-btn" onClick={() => run('connto')}>
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
                        onMouseDown={() => turn('go_left')}
                        onMouseUp={stop}
                    >
                        GoLeft
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => run('go')}
                        onMouseUp={stop}
                    >
                        Go
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => turn('go_right')}
                        onMouseUp={stop}
                    >
                        GoRight
                    </button>
                </div>
                <div className="controller-line">
                    <button
                        className="controller-btn"
                        onMouseDown={() => run('left')}
                        onMouseUp={stop}
                    >
                        Left
                    </button>
                    <button className="controller-btn" onClick={() => run('stop')}>
                        Stop
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => run('right')}
                        onMouseUp={stop}
                    >
                        Right
                    </button>
                </div>
                <div className="controller-line">
                    <button
                        className="controller-btn"
                        onMouseDown={() => turn('back_left')}
                        onMouseUp={stop}
                    >
                        BackLeft
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => run('back')}
                        onMouseUp={stop}
                    >
                        Back
                    </button>
                    <button
                        className="controller-btn"
                        onMouseDown={() => turn('back_right')}
                        onMouseUp={stop}
                    >
                        BackRight
                    </button>
                </div>
            </div>
        </main>
    );
}


