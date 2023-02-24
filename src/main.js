const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {

  let tbl = document.getElementById("main_table");

  var element = document.getElementById("t_body");
  element.remove();

  let tblBody = document.createElement("tbody");
  tblBody.setAttribute("id", "t_body");

  for (let i = 0; i < 32; i++) {
    const row = document.createElement("tr");

    for (let j = 0; j < 16; j++) {
      const cell = document.createElement("td");
      const cellText = document.createTextNode(`None`);

      cell.appendChild(cellText);
      row.appendChild(cell);
    }

    // add the row to the end of the table body
    tblBody.appendChild(row);

    if (i == 16) {
      const row = document.createElement("tr");
      for (let j = 0; j < 16; j++) {
        const cell = document.createElement("td");
        const cellText = document.createTextNode(` -----\n-----\n----- `);
        cell.appendChild(cellText);
        row.appendChild(cell);
      }
      tblBody.appendChild(row);
    }
  }
  tbl.appendChild(tblBody);

  await invoke("open_file", {path: "C:\\Users\\nikiy\\Documents\\Документ1.docx"});
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => {greet(); });
});


let lastKnownScrollPosition = 0;
let ticking = false;

localStorage.setItem("last_pos", 0)

// async function editTable() {
//
//   let tbl = document.getElementById("main_table");
//   console.log("Вывод индекса");
//   console.log(localStorage.getItem("last_pos"));
//
//   var element = document.getElementById("t_body");
//   element.remove();
//
//   let chunks = await invoke("get_bytes_chunks", {})
//   console.log(chunks);
//   let tblBody = document.createElement("tbody");
//   tblBody.setAttribute("id", "t_body");
//
//   for (let i = 0; i < 32; i++) {
//     const row = document.createElement("tr");
//
//     for (let j = 0; j < 16; j++) {
//       const cell = document.createElement("td");
//       const pos = localStorage.getItem("last_pos");
//       let node = chunks[i+(16*pos)][j]
//       const cellText = document.createTextNode(`${node.toString(16)}`);
//       cell.appendChild(cellText);
//       row.appendChild(cell);
//     }
//
//     // add the row to the end of the table body
//     tblBody.appendChild(row);
//
//     if (i == 16) {
//       const row = document.createElement("tr");
//       for (let j = 0; j < 16; j++) {
//         const cell = document.createElement("td");
//         const cellText = document.createTextNode(` -----\n-----\n----- `);
//         cell.appendChild(cellText);
//         row.appendChild(cell);
//       }
//       tblBody.appendChild(row);
//     }
//   }
//   tbl.appendChild(tblBody);
// }

async function editTable() {

  let tbl = document.getElementById("main_table");

  let tbl_body = document.getElementById("t_body");
  let tbl_children = tbl_body.children;
  console.log(tbl_children);

  let chunks = await invoke("get_bytes_chunks", {})
  console.log(chunks);

  for (let i = 0; i < 32; i++) {

    for (let j = 0; j < 16; j++) {
      tbl_children[i][j].children[0].remove();("last_pos");

      const pos = localStorage.getItem
      let node = chunks[i+(16*pos)][j]
      const cellText = document.createTextNode(`${node.toString(16)}`);
      tbl_children[i][j].appendChild(cellText);
    }
  }
}


async function doSomething(scrollPos) {
  console.log("Скролл",scrollPos, localStorage.getItem("scrollPos"));

  //if (parseInt(localStorage.getItem("scrollPos"))+40 < parseInt(scrollPos)) {
  //}

  localStorage.setItem("scrollPos", scrollPos);
  //if (scrollPos > 285) {
  //if (scrollPos > 20) {
      await editTable();
      window.scroll(0,0);
      //window.scrollTo(0, 0);
      localStorage.setItem("last_pos", parseInt(localStorage.getItem("last_pos")) + 1);
  //}

}

document.addEventListener("scroll", (event) => {
  lastKnownScrollPosition = window.scrollY;

  if (!ticking) {
    window.requestAnimationFrame(() => {
      doSomething(lastKnownScrollPosition);
      ticking = false;
    });

    ticking = true;
  }
});
