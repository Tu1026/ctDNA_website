// TODO: Document that cfDNA concentrations should be quantified with Qubit.

function $(query) {
    return document.querySelector(query);
}

function $$(query) {
    return document.querySelectorAll(query);
}
HTMLElement.prototype.clear = function() {
    this.innerHTML = "";
};
HTMLElement.prototype.append = function(html) {
    this.insertAdjacentHTML("beforeend", html);
};

// function async_get(url, success_handler) {
//     var request = new XMLHttpRequest();
//     request.open("GET", url, true);
//     request.onload = function() {
//         if (this.status == 200 && success_handler) success_handler(this.response);
//     };
//     request.send();
// }

function clear_prediction(d) {
    $("#results").innerHTML = "";
    $("#neighbors").innerHTML = "";
}

function async_get(url, success_handler) {
    const XHR = new XMLHttpRequest();
    const FD = new FormData();

    // Push our data into our FormData object
    for (const [name, value] of Object.entries(data)) {
        FD.append(name, value);
    }

    // Define what happens on successful data submission
    XHR.addEventListener("load", (event) => {
        alert("Yeah! Data sent and response loaded.");
    });

    // Define what happens in case of error
    XHR.addEventListener("error", (event) => {
        alert("Oops! Something went wrong.");
    });

    // Set up our request
    XHR.open("POST", "https://example.com/cors.php");

    // Send our FormData object; HTTP headers are set automatically
    XHR.send(FD);
}

// TODO: Use bootstrapping to estimate prediction interval for KNN model.
function display_prediction(d) {
    let lines = d.split("\n");
    let estimate = lines.shift(); // First line gives ctDNA% estimate
    if (estimate == "0.0") {
        estimate = "0 - 2";
    }
    $(
        "#results"
    ).innerHTML = `Predicted ctDNA%<br><span id="estimate">${estimate}%</span><br>for a patient with progressive disease`;

    // Populate the nearest neighbors table
    //     let neighbors = lines.filter((line) => line.trim() != "");

    //     let html = `<h1>Estimate is based on these ${neighbors.length} patients with similar characteristics</h1>`;
    //     html += `<table><tr>
    //     <td>cfDNA yield (ng/mL)</td>
    //     <td>PSA (ng/mL)</td>
    //     <td>LDH (ULN)</td>
    // @@ -237,63 +475,74 @@ <h1>Who created this website?</h1>
    //     <td>ctDNA (%)</td>
    // </tr>`;

    //     for (let line of neighbors) {
    //         html += `<tr>`;
    //         for (let col of line.split("\t")) {
    //             html += `<td>${col}</td>`;
    //         }
    //         html += `</tr>`;
    //     }
    //     html += `</table>`;

    //     $("#neighbors").innerHTML = html;
    //     $("#neighbors").classList.remove("hidden");
}

document.addEventListener("DOMContentLoaded", function(e) {
    $("#predict_button").addEventListener("click", (e) => {
        // let cfdna_yield = $("#cfdna_yield > input").value;
        // let psa = $("#psa > input").value;
        // let ldh = $("#ldh > input").value;
        // let alp = $("#alp > input").value;
        // let ecog = $("#ecog .selected").textContent;
        // let bone_mets = $("#bone_mets .selected").textContent;
        // let lung_mets = $("#lung_mets .selected").textContent;
        // let liver_mets = $("#liver_mets .selected").textContent;

        // let query = `/predict?`;
        // if (bone_mets != "N/A") {
        //     query += `bone_mets=${bone_mets == "Yes"}&`;
        // }
        // if (lung_mets != "N/A") {
        //     query += `lung_mets=${lung_mets == "Yes"}&`;
        // }
        // if (liver_mets != "N/A") {
        //     query += `liver_mets=${liver_mets == "Yes"}&`;
        // }
        // if (cfdna_yield != "") {
        //     query += `cfdna_yield=${cfdna_yield}&`;
        // }
        // if (psa != "") {
        //     query += `psa=${psa}&`;
        // }
        // if (ldh != "") {
        //     query += `ldh=${ldh}&`;
        // }
        // if (alp != "") {
        //     query += `alp=${alp}&`;
        // }
        // if (ecog != "N/A") {
        //     query += `ecog=${ecog}&`;
        // }

        // // Remove the trailing ampersand character and send the query
        // query = query.replace(/&$/, "");
        // if (query.endsWith("?") == false) {
        //     async_get(query, display_prediction);
        // }
        async_get("/predict", display_prediction);
    });

    // Make the radio buttons functional
    for (let radio of $$(".radio")) {
        radio.addEventListener("click", (e) => {
            if (e.target.parentElement.classList.contains("radio")) {
                for (let child of e.target.parentElement.children) {
                    child.classList.remove("selected");
                }
                e.target.classList.add("selected");
                clear_prediction();
            }
        });
    }
});