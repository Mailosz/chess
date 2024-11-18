
import {DocBuilder} from "./www/js/utils/DocBuilder.js";


export class ChessBoard extends HTMLElement {

    #dragging = null;
    #available_moves = [];

    constructor() {
        super();

        const db = new DocBuilder(this.ownerDocument);

        

        let light = true;
        for (let i = 0; i < 64; i++) {
            if (i % 8 != 0) light = !light;
    
            const square = db.div()
                .class("square", light ? "light" : "dark")
                .event("click", (event) => {event.currentTarget.classList.toggle("highlighted")})
                .event("dragover", (event) => {
                    if (this.dragging != null) {
                        event.preventDefault();
                        event.dataTransfer.dropEffect = "move";
                        event.target.classList.add("dropover");
                    }
                })
                .event("dragleave", (event) => {
                    event.currentTarget.classList.remove("dropover");
                })
                .event("drop", (event) => {
                    event.currentTarget.classList.remove("dropover");

                    const position = event.currentTarget.position;

                    if (this.#available_moves.includes(position)) {
                        console.log("drop  " + this.dragging.parentElement.position + " on " + position);

                        this.dragging.parentElement.removeChild(this.dragging);
    
                        event.currentTarget.appendChild(this.dragging);
                    } else {
                        console.log("Forbidden move: " + this.dragging.parentElement.position + " to " + position);
                    }
                    this.#available_moves = [];

                })
                .get();
    
            square.position = i;
    
            this.appendChild(square);
    
            // if (Math.floor(i / 8) == 1) {
    
            //     square.appendChild(this.#createPiece("pawn", "white"));
            // }
    
            // if (Math.floor(i / 8) == 7) {
            //     square.appendChild(this.#createPiece("pawn", "black"));
            // }
    
        }
    }

    #createPiece(type, color) {
        const db = new DocBuilder(this.ownerDocument);
        return db.div().class("piece", type, color)
            .event("dragstart", (event) => {
                
                const position = event.target.parentElement.position;
                this.dragging = event.target;

                this.#available_moves = this.getAvailableMoves(position);

                console.log("Start: " + position);
                console.log("Available moves: " + this.#available_moves);
            })
            .event("dragend", (event) => {
                this.#dragging = null;
            })
            .attr("draggable", "true")
            .get();
    }

    getAvailableMoves = (pos) => {
        let res = [];
        for (let i = 0; i < 64; i++) {
            res.push(i);
        }
        return res;
    }

    
    setChessboard(boardString) {

        let board = boardString.split(",");

        let square = this.firstElementChild;
        for (let i = 0; i < 64; i++) {
            //remove existing 
            let oldPieces = square.getElementsByClassName("piece");
            for (const p of oldPieces) {
                square.removeChild(p);
            }

            switch (board[square.position]) {
                case "PW":
                    square.appendChild(this.#createPiece("pawn", "white"));
                    break;
                case "KW":
                    square.appendChild(this.#createPiece("king", "white"));
                    break;
                case "QW":
                    square.appendChild(this.#createPiece("queen", "white"));
                    break;
                case "RW":
                    square.appendChild(this.#createPiece("rook", "white"));
                    break;
                case "BW":
                    square.appendChild(this.#createPiece("bishop", "white"));
                    break;                   
                case "NW":
                    square.appendChild(this.#createPiece("knight", "white"));
                    break;                        

                case "PB":
                    square.appendChild(this.#createPiece("pawn", "black"));
                    break;
                case "KB":
                    square.appendChild(this.#createPiece("king", "black"));
                    break;
                case "QB":
                    square.appendChild(this.#createPiece("queen", "black"));
                    break;
                case "RB":
                    square.appendChild(this.#createPiece("rook", "black"));
                    break;
                case "BB":
                    square.appendChild(this.#createPiece("bishop", "black"));
                    break;                   
                case "NB":
                    square.appendChild(this.#createPiece("knight", "black"));
                    break;   
            };

            square = square.nextElementSibling;
        }
    }
} 


window.customElements.define("chess-board", ChessBoard);