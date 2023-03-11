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
        <p>
          Edit <code>src/App.jsx</code> and save to reload.
        </p>

        <button onClick={openFile}>Click Me</button>
        <Show when={openFileState() == true} fallback={<div>Загрузка...</div>}>
          <CreateTable />
        </Show>

      </header>
    </div>
  );
}


function CreateTable() {
  const [PrevChunk, setPrevChunk] = createSignal(new Array(256));
  const [CurrentChunk, setCurrentChunk] = createSignal(new Array(256));
  const [NextChunk, setNextChunk] = createSignal(new Array(256));

  new Promise(
    function(resolve, reject) {
      const chunk = invoke("next_chunk", {});
      resolve(chunk);
    }
  ).then(function(result) {
    console.log(result);
    setPrevChunk(result)
  });

  new Promise(
    function(resolve, reject) {
      const chunk = invoke("next_chunk", {});
      resolve(chunk);
    }
  ).then(function(result) {
    console.log(result);
    setCurrentChunk(result);
  });

  new Promise(
    function(resolve, reject) {
      const chunk = invoke("next_chunk", {});
      resolve(chunk);
    }
  ).then(function(result) {
    console.log(result);
    setNextChunk(result)
  });

  //console.log(PrevChunk(), CurrentChunk(), NextChunk());
  const table = (
    <div style="height:250px; overflow: auto;">
      <div class = {styles.bytes_table} id = "bytes_table" style="height:2000px">
        <div id = "free_space" style="height: 0px" />
        <div style="height: 200px">
          <PrevChunk />
        </div>
        <div style="height: 200px">
          <CurrentChunk/>
        </div>
        <div style="height: 200px">
        
        </div>
      </div>
    </div>
  );

  const [PrevChunkScrollPos, setPrevChunkScrollPos] = createSignal(0);
  const [heightFreeSpace, setHeightFreeSpace] = createSignal(0);

  //const free_space = table.getElementById("free_space");

  //table.scrollTo(0, 200);

  table.addEventListener("scroll", (event) => {

    if (table.scrollTop > PrevChunkScrollPos() + 200) {
      for (const child of table.firstElementChild.children) {
        if (child.id == "free_space") {
          setHeightFreeSpace(heightFreeSpace() + 200);
          child.style.height = heightFreeSpace() + "px";
          table.scrollTo(0, table.scrollTop-200);

          setPrevChunk(CurrentChunk());
          setCurrentChunk(NextChunk());

          new Promise(
            function(resolve, reject) {
              invoke("next_chunk", {})
            }
          ).then(setNextChunk);

        }
      }

      console.log("next chunk");
      setPrevChunkScrollPos(table.scrollTop);
    }

    // if (table.scrollTop > PrevChunkScrollPos() + 200) {
    //   setPrevChunkScrollPos(table.scrollTop)
    //
    //   for (const child of table.firstElementChild.children) {
    //       if (child.id == "free_space") {
    //           setHeightFreeSpace(heightFreeSpace() + 200);
    //           child.style.height = heightFreeSpace() + "px";
    //       }
    //   }
    //
    //   console.log("next_chunk");
    // }
    // else if (table.scrollTop < PrevChunkScrollPos() - 200) {
    //   setPrevChunkScrollPos(table.scrollTop);
    //
    //   for (const child of table.firstElementChild.children) {
    //       if (child.id == "free_space") {
    //         setHeightFreeSpace(heightFreeSpace() - 200);
    //         child.style.height = heightFreeSpace() + "px";
    //       }
    //   }
    //
    //   console.log("prev_chunk");
    // }

  });

  return table;
}

// function OldCreateTable() {
//
//   const [currentChunk, setCurrentChunk] = createSignal(new Array(256));
//   const [nextChunk, setNextChunk] = createSignal(new Array(256));
//   const [prevChunk, setPrevChunk] = createSignal(new Array(256));
//
//   let lastKnownScrollPosition = 0;
//   let ticking = false;
//
//   document.addEventListener("scroll", (event) => {
//   lastKnownScrollPosition = window.scrollY;
//
//   const next_chunk = async () => setChunk( await invoke("next_chunk_split", {}) );
//
//   if (!ticking) {
//     window.requestAnimationFrame(() => {
//       console.log(lastKnownScrollPosition);
//
//       if (lastKnownScrollPosition > 380) {
//         lazy(next_chunk())
//         window.scrollTo(0, 350)
//       };
//
//       ticking = false;
//     });
//
//     ticking = true;
//   }
// });
//
//   // setInterval(async () => {
//   //   let new_arr = await invoke("next_chunk", {});
//   //   setChunk(new_arr);
//   //   console.log(chunk());
//   //   //window.resizeTo(20000, 20000);
//   // }, 5000);
//
//   let t =  <div class={styles.bytes_table} id = "bytes_table">
//   <button onClick = {async () => setCurrentChunk( await invoke("next_chunk_split", {}))} >NextChunk</button>
//   <table>
//     <tbody>
//
//     <For each={currentChunk()} fallback={<div>Загрузка...</div>}>
//       {(item_i, index_i) => (
//         <tr>
//         <td>Line{index_i()}</td>
//
//         <For each={item_i}>
//         {(item_j, index_j) => {
//           let byte = parseInt(item_j).toString(16);
//
//           if (byte.length == 1) {
//             byte = "0" + byte;
//           }
//
//           return <td>{byte}</td>
//         }
//
//       }
//         </For>
//         </tr>
//       )}
//     </For>
//
//     <tr>
//     <td>None</td>
//     </tr>
//     <tr>
//     <td>None</td>
//     </tr>
//     <tr>
//     <td>None</td>
//     </tr>
//
//     </tbody>
//   </table>
//   </div>
// }

export default App;
