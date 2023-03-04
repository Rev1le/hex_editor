import logo from './logo.svg';
import styles from './App.module.css';
import { createSignal, createEffect, lazy } from 'solid-js';
import { invoke } from '@tauri-apps/api'

function App() {

  const [openFileState, setOpenFileState] = createSignal(false);

  createEffect(() => {
    console.log("Open file state: ", openFileState());
  });

  const openFile = async () => {
    setOpenFileState( await invoke("open_file", {chunk_size: 256}) )
  };



  return (

    <div class={styles.App}>

      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.jsx</code> and save to reload.
        </p>

        <button onClick={openFile}>Click Me</button>

        <Show when={openFileState() == true} fallback={<div>Загрузка...</div>}>
        <div class = {styles.bytes_table}>
          <p>Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1\nТаблица1Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1\nТаблица1Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1\nТаблица1Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1\nТаблица1Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1\nТаблица1Таблица1Таблица1Таблица1Таблица1Таблица1Таблица1\n</p>
          <p>Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2\nТаблица2Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2\nТаблица2Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2\nТаблица2Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2\nТаблица2Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2\nТаблица2Таблица2Таблица2Таблица2Таблица2Таблица2Таблица2\n</p>
          <p>Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3\nТаблица3Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3\nТаблица3Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3\nТаблица3Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3\nТаблица3Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3\nТаблица3Таблица3Таблица3Таблица3Таблица3Таблица3Таблица3\n</p>
          </div>
        </Show>

      </header>
    </div>
  );
}


function CreateTable() {



  const [currentChunk, setCurrentChunk] = createSignal(new Array(256));
  const [nextChunk, setNextChunk] = createSignal(new Array(256));
  const [prevChunk, setPrevChunk] = createSignal(new Array(256));

  let lastKnownScrollPosition = 0;
  let ticking = false;

  document.addEventListener("scroll", (event) => {
  lastKnownScrollPosition = window.scrollY;

  const next_chunk = async () => setChunk( await invoke("next_chunk_split", {}) );

  if (!ticking) {
    window.requestAnimationFrame(() => {
      console.log(lastKnownScrollPosition);

      if (lastKnownScrollPosition > 380) {
        lazy(next_chunk())
        window.scrollTo(0, 350)
      };

      ticking = false;
    });

    ticking = true;
  }
});

  // setInterval(async () => {
  //   let new_arr = await invoke("next_chunk", {});
  //   setChunk(new_arr);
  //   console.log(chunk());
  //   //window.resizeTo(20000, 20000);
  // }, 5000);

  let t =  <div class={styles.bytes_table} id = "bytes_table">
  <button onClick = {async () => setCurrentChunk( await invoke("next_chunk_split", {}))} >NextChunk</button>
  <table>
    <tbody>

    <For each={currentChunk()} fallback={<div>Загрузка...</div>}>
      {(item_i, index_i) => (
        <tr>
        <td>Line{index_i()}</td>

        <For each={item_i}>
        {(item_j, index_j) => {
          let byte = parseInt(item_j).toString(16);

          if (byte.length == 1) {
            byte = "0" + byte;
          }

          return <td>{byte}</td>
        }

      }
        </For>
        </tr>
      )}
    </For>

    <tr>
    <td>None</td>
    </tr>
    <tr>
    <td>None</td>
    </tr>
    <tr>
    <td>None</td>
    </tr>

    </tbody>
  </table>
  </div>
}

export default App;
