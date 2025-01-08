import { useEffect, useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
  const [addr, setaddr] = useState('');
  async function exec(cmd: string, addr: string) {
    await invoke(cmd, { addr });
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
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <button onMouseDown={() => exec('go', addr)} onMouseUp={stop}>
        Go
      </button>
      <button onMouseDown={() => exec('back', addr)} onMouseUp={stop}>
        Back
      </button>

      <button onMouseDown={() => exec('left', addr)} onMouseUp={stop}>
        Left
      </button>
      <button onMouseDown={() => exec('right', addr)} onMouseUp={stop}>
        Right
      </button>
      <button onClick={() => exec('stop', addr)}>Stop</button>
      <button onClick={() => exec('connto', addr)}>Conn</button>
      <input onChange={(e) => setaddr(e.target.value)}></input>
    </main>
  );
}

export default App;
