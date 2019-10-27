import * as wasm from "test-encryption";

let dataset = [];
const cryptos = ["c2chacha20_encrypt_decrypt",
    "blowfish_cbc_encrypt_decrypt",
    "rust_crypto_aes_encrypt_decrypt",
    "aes_256_gcm_siv_encrypt_decrypt",
    "rust_crypto_blowfish_encrypt_decrypt"];
const colors =  ['rgba(255, 159, 64, 1)',
    'rgba(54, 162, 235, 1)',
    'rgba(255, 206, 86, 1)',
    'rgba(75, 192, 192, 1)',
    'rgba(153, 102, 255, 1)',
   ];
var testingAlphabet = false;


const runTests = () => {
    var key = "aklsöjdfkölasdjfk";
    var text = document.getElementById("text1").value;
    var times = document.getElementById("number1").value;
    var times2 = document.getElementById("number2").value;
    var runTimes = new Map();
    //TODO add pairs to map dynamically, depending on the cryptos array OR replace the usage of cryptos array with the map
    runTimes.set("c2chacha20_encrypt_decrypt", []);
    runTimes.set("blowfish_cbc_encrypt_decrypt", []);
    runTimes.set("rust_crypto_aes_encrypt_decrypt", []);
    runTimes.set("aes_256_gcm_siv_encrypt_decrypt", []);
    runTimes.set("rust_crypto_blowfish_encrypt_decrypt", []);
    for (let j = 0; j<times2;j++){
        for (let i = 0; i<cryptos.length;i++){
            var diff = -1;
            var beforeTest = new Date().getTime();
            switch (cryptos[i]) {
                case "c2chacha20_encrypt_decrypt":
                    for (let i = 0; i < times; i++) {
                        wasm.c2chacha20_encrypt_decrypt(text, key)
                    }
                    break;
                case "blowfish_cbc_encrypt_decrypt":
                    for (let i = 0; i < times; i++) {
                        wasm.blowfish_cbc_encrypt_decrypt(text, key)
                    }
                    break;
                case "rust_crypto_aes_encrypt_decrypt":
                    for (let i = 0; i < times; i++) {
                        wasm.rust_crypto_aes_encrypt_decrypt(text, key)
                    }
                    break;
                case "aes_256_gcm_siv_encrypt_decrypt":
                    for (let i = 0; i < times; i++) {
                        wasm.aes_256_gcm_siv_encrypt_decrypt(text, key)
                    }
                    break;
                case "rust_crypto_blowfish_encrypt_decrypt":
                    for (let i = 0; i < times; i++) {
                        wasm.rust_crypto_blowfish_encrypt_decrypt(text, key)
                    }
                    break;
            }
            var afterTest = new Date().getTime();
            var diff = afterTest-beforeTest;
            var currArray = runTimes.get(cryptos[i]);
            currArray.push(diff)
            runTimes.set(cryptos[i],currArray);
        }
    }


        //generate table
        var avgs = [];
        var table = "<table> <tr><th>Methode</th><th>avg time [ms]</th><th>max time [ms]</th><th>min time [ms]</th></tr>";
        for (let i = 0; i<cryptos.length;i++){
            var sum = 0;
            var array = runTimes.get(cryptos[i]);
            for( var j = 0; j < array.length; j++ ){
                sum += parseInt( array[j], 10 );
            }
            var avg = sum/array.length;
            avgs.push(avg);
            var min = Math.min.apply(Math,array);
            var max = Math.max.apply(Math,array);
            table += "<tr><td>" + cryptos[i] + "</td><td> "+ avg +"</td><td> "+ max +"</td><td> "+ min +"</td></tr>";

        }
        table += "</table>";
    dataset = [avgs[0], avgs[1], avgs[2], avgs[3], avgs[4]];
    if(testingAlphabet==false){
        document.getElementById("tbl").innerHTML = table;

        // create ChartJS stuff
        var ctx = document.getElementById('myChart').getContext('2d');

        var myChart = new Chart(ctx, {
            type: 'bar',
            data: {
                labels: ['C2', 'Blowfish', 'rust-crypto-aes', 'aes_256', 'rust-crypto-blowfish'],
                datasets: [{
                    label: 'avg ms',
                    //TODO insert data dynamically
                    data: [avgs[0], avgs[1], avgs[2], avgs[3], avgs[4]],
                    backgroundColor: [
                        //TODO get colors from global constant
                        'rgba(255, 99, 132, 1)',
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 206, 86, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(153, 102, 255, 1)',
                        'rgba(255, 159, 64, 1)'
                    ],
                    borderWidth: 1
                }]
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                }
            }
        });
    }




};
const testAlphabet  = () => {
    testingAlphabet = true;
    var alphabet = "abcdefghijklmnopqrstuvwxyzäöüßABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜ1234567890!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ".split("");

    var upperAlphabet = "abcdefghijklmnopqrstuvwxyz".toUpperCase();
    upperAlphabet = upperAlphabet.split("");
    alphabet.push.apply(upperAlphabet);

    var text = document.getElementById("text1").value;

    var datasets2 = new Map();
    for (let i = 0; i<alphabet.length;i++){
        document.getElementById("text1").value = alphabet[i];
        runTests();
        datasets2.set(alphabet[i],dataset);
    }

    var ctx = document.getElementById('myChart').getContext('2d');
    var label = [];
    var data = [];
    //Prepare data for each crypto algorithm
    for(let i = 0; i<cryptos.length;i++){
        var tmp = {
            label: cryptos[i],
            backgroundColor: colors[i],
            borderColor: colors[i],
            data: [],
            fill: false,
        };
        data.push(tmp);
    }
    //fill actual data into the data array
    for (var [key, value] of datasets2.entries()) {
        console.log(key + ' = ' + value);
        for(let i = 0; i<cryptos.length;i++){
            data[i].data.push(value[i]);
        }
        label.push(key);
    }
    console.log(data);
    var myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: label,
            datasets: data
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero: true
                    }
                }]
            }
        }
    });
    document.getElementById("tbl").innerHTML = "";
    testingAlphabet = false;
}

$(document).ready(function() {
    $("#run").click(runTests);
    $("#runAlphabet").click(testAlphabet);
});
