import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [addr, setaddr] = useState('');
  async function exec(cmd: string, addr: string) {
    await invoke(cmd, { addr });
  }
  return (
    <main className="container">
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
      <button onClick={() => exec("go",addr)}>Go</button>
      <button onClick={() => exec("back",addr)}>Back</button>
      <button onClick={() => exec("stop",addr)}>Stop</button>
      <button onClick={() => exec("left",addr)}>Left</button>
      <button onClick={() => exec("right",addr)}>Right</button>
      <button onClick={() => exec("connto",addr)}>Conn</button>
      <input onChange={(e) => setaddr(e.target.value)}></input>
    </main>
  );
}

export default App;
