import * as wasm from "test-encryption";
import _ from 'lodash';
import process from 'process';

const benchmark = require('benchmark');
const Benchmark = benchmark.runInContext({ _, process });
window.Benchmark = Benchmark;

Benchmark.options.maxTime = 1;


var dataset = new Map();
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
var whatIsTested = null;
const key_iv_setup_test = () => {
    var key_iv_suite = new Benchmark.Suite;
    var key = document.getElementById("key").value;
    var iv = document.getElementById("iv").value;
    var times = 100;
    key_iv_suite.on('cycle', function(event) {
        console.log(String(event.target));
    });
    key_iv_suite.add("c2chacha20_key_iv_setup", function (){
        for (let i = 0; i < times; i++) {
            wasm.c2chacha20_key_iv_setup(key,iv);
        }
    });
    key_iv_suite.add("blowfish_cbc_key_iv_setup", function (){
        for (let i = 0; i < times; i++) {
            wasm.blowfish_cbc_key_iv_setup(key,iv);
        }
    });
    key_iv_suite.add("rust_crypto_aes_key_iv_setup", function (){
        for (let i = 0; i < times; i++) {
            wasm.rust_crypto_aes_key_iv_setup(key,iv);
        }
    });
    key_iv_suite.add("aes_256_gcm_siv_key_iv_setup", function (){
        for (let i = 0; i < times; i++) {
            wasm.aes_256_gcm_siv_key_iv_setup(key,iv);
        }
    });
    key_iv_suite.add("rust_crypto_blowfish_key_iv_setup", function (){
        for (let i = 0; i < times; i++) {
            wasm.rust_crypto_blowfish_key_iv_setup(key);
        }
    });
    key_iv_suite.on('complete', function() {
        var entry = {};
        for (let i = 0; i < this.length; i++) {
            entry[this[i].name] = this[i].stats.mean;
        }
        dataset.set("key_iv_setup", entry);
    });
    // run async
    key_iv_suite.run({ 'async': false });
}

const encrypt_decrypt_test = () => {

    var key = document.getElementById("key").value;
    var iv = document.getElementById("iv").value;
    var text = document.getElementById("text1").value;
    wasm.aes_256_gcm_siv_encrypt_decrypt(text,key,iv);
    var encrypt_decrypt_suite = new Benchmark.Suite;
    encrypt_decrypt_suite.on('cycle', function(event) {
        console.log(String(event.target));
    });
    encrypt_decrypt_suite.add("c2chacha20_encrypt_decrypt", function (){
        wasm.c2chacha20_encrypt_decrypt(text,key,iv);
    });
    encrypt_decrypt_suite.add("blowfish_cbc_encrypt_decrypt", function (){
        wasm.blowfish_cbc_encrypt_decrypt(text,key,iv);
    });
    encrypt_decrypt_suite.add("rust_crypto_aes_encrypt_decrypt", function (){
        wasm.rust_crypto_aes_encrypt_decrypt(text,key,iv);
    });
    encrypt_decrypt_suite.add("aes_256_gcm_siv_encrypt_decrypt", function (){
        wasm.aes_256_gcm_siv_encrypt_decrypt(text,key,iv);
    });
    encrypt_decrypt_suite.add("rust_crypto_blowfish_encrypt_decrypt", function (){
        wasm.rust_crypto_blowfish_encrypt_decrypt(text,key);
    });
    encrypt_decrypt_suite.on('complete', function() {
        var entry = new Map();
        for (let i = 0; i < this.length; i++) {
            entry.set(this[i].name, this[i].stats.mean);
        }
        dataset.set(text,entry);
    });
    // run async
    encrypt_decrypt_suite.run({ 'async': false });
}
const testing = () => {
   // encrypt_decrypt_test();
   // visualizeDataset();
    //key_iv_setup_test();
}
const testAlphabet = () => {
    whatIsTested = "alphabet";
    var alphabet = "abcdefghijklmnopqrstuvwxyzäöüßABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜ1234567890!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ".split("");
    for (let i = 0; i<alphabet.length;i++){
        document.getElementById("text1").value = alphabet[i];
        encrypt_decrypt_test();
    }
    visualizeDataset();
}
const testAlphabet16 = () => {
    whatIsTested = "alphabet";
    var alphabet = "abcdefghijklmnopqrstuvwxyzäöüßABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜ1234567890!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ".split("");
    for (let i = 0; i<alphabet.length;i++){
        document.getElementById("text1").value = alphabet[i].repeat(16);
        encrypt_decrypt_test();
    }
    visualizeDataset();
}
const testRandString = () => {
    whatIsTested = "randString";
    var alphabet = "abcdefghijklmnopqrstuvwxyzäöüßABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜ1234567890!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ";
    for (let i = 1; i<=64;i++){
        //generate random string
        //source: https://stackoverflow.com/questions/1349404/generate-random-string-characters-in-javascript
        var str = "";
        for (var j = 0; j < i; j++) {
            str += alphabet.charAt(Math.floor(Math.random() * alphabet.length));
        }
        document.getElementById("text1").value = str;
        encrypt_decrypt_test();
    }
    visualizeDataset();
}

const testSetup = () => {
    key_iv_setup_test();
}
const visualizeSetup = () => {
    
}

//when called there should not be key_iv results in the resultset
const visualizeDataset = () => {
    var data = [];
    var label = [];

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
    for (var [key, value] of dataset.entries()) {
        console.log(key + ' = ' + value);
        for (let i = 0; i < cryptos.length; i++) {
            data[i].data.push(value.get(data[i].label));
        }
        if(whatIsTested=="alphabet"){
            label.push(key[0]);
        }
        else if (whatIsTested == "randString"){
            label.push(key.length);
        }

    }

    var ctx = document.getElementById('myChart').getContext('2d');
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
}


$(document).ready(function() {
    $("#runAlphabet").click(testAlphabet);
    $("#bench").click(testing);
});
