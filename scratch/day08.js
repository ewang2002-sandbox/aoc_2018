class Node {
    constructor() {
        this.childNodes = [];
        this.metaData = [];
    }
}

// PART 1

// Parent function
function getNodes(input) {
    let node = new Node();
    run(input, node);
    return node;
}

// "node" refers to the current node to deal with.
// The input
function run(input, node) {
    // No more input to process.
    if (input.length === 0) return;

    // Get child node + metadata count.
    let numChildNodes = input.shift();
    let numMetaData = input.shift();

    // If no child nodes, then put metadata in the current node and leave
    if (numChildNodes === 0) {
        while (numMetaData > 0) {
            node.metaData.push(input.shift());
            numMetaData--;
        }
        return;
    }

    // Otherwise, create a new node for each child node then attach to the current node.
    while (numChildNodes > 0) {
        const newNode = new Node();
        run(input, newNode);
        node.childNodes.push(newNode);
        numChildNodes--;
    }

    // Then, add the meta data to the current node.
    while (numMetaData > 0) {
        node.metaData.push(input.shift());
        numMetaData--;
    }
}

//const input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".split(" ").map(x => Number.parseInt(x, 10));
const input = document.body.innerText.split(" ").map(x => Number.parseInt(x, 10));
const res = getNodes(input);