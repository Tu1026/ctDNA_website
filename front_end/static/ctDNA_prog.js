// TODO: Document that cfDNA concentrations should be quantified with Qubit.

const prediction_form = document.forms.sample;

prediction_form.addEventListener("submit", handleSubmit);

function handleSubmit(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    queryString = new URLSearchParams(formData);

    queryString = sanitize_form_fields(queryString);

    queryString = queryString.toString();

    console.log(queryString);

    if (check_input_valid(queryString)) {
        console.log("was valid");
        var request = new XMLHttpRequest();
        request.open("GET", "/predict?" + queryString.toString(), true);
        request.onload = function() {
            if (this.status == 200) {
                display_prediction(this.response);
            }
        };
        request.send();
    }
}

//Don't send empty fields in the query string
function sanitize_form_fields(queryStringObj) {
    let keysForDel = [];
    queryStringObj.forEach((value, key) => {
        if (value == "") {
            keysForDel.push(key);
        }
    });
    //remove empty query string

    keysForDel.forEach((key) => {
        queryStringObj.delete(key);
    });
    return queryStringObj;
}

function count(str, find) {
    return str.split(find).length - 1;
}

//Check if the input string is valid for prediction
// TODO add more checks for invalid input formats that might mess up the server
function check_input_valid(queryString) {
    //First check if at least 4 fields are filled
    if (count(queryString, "=") < 4) {
        display_error("please input at least 4 fields");
        return false;
    }
    return true;
}

function revmoeEmptyQueryField(queryString) {
    let keysForDel = [];
    queryString.forEach((value, key) => {
        if (value == "") {
            keysForDel.push(key);
        }
    });
    //remove empty query string
    keysForDel.forEach((key) => queryString.delete(key));
}

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

function display_error(error) {
    $(
        "#results"
    ).innerHTML = `Cannot predict the patient ctDNA% <strong id="error"> ${error} </strong>`;
}

function clear_prediction() {
    $("#results").innerHTML = "";
    $("#neighbors").innerHTML = "";
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
}