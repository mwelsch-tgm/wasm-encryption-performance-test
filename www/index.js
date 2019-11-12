import * as wasm from "test-encryption";
import _ from 'lodash';
import process from 'process';

const benchmark = require('benchmark');
const Benchmark = benchmark.runInContext({ _, process });
window.Benchmark = Benchmark;

Benchmark.options.maxTime = 1;


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


const key_iv_setup_test = () => {
    var key_iv_suite = new Benchmark.Suite;
    var key = document.getElementById("key").value;
    var iv = document.getElementById("iv").value;
    key_iv_suite.on('cycle', function(event) {
        console.log(String(event.target));
    });
    key_iv_suite.add("c2chacha20_key_iv_setup", function (){
        wasm.c2chacha20_key_iv_setup(key);
    });
    key_iv_suite.add("blowfish_cbc_key_iv_setup", function (){
        wasm.blowfish_cbc_key_iv_setup(key);
    });
    key_iv_suite.add("rust_crypto_aes_key_iv_setup", function (){
        wasm.rust_crypto_aes_key_iv_setup(key);
    });
    key_iv_suite.add("aes_256_gcm_siv_key_iv_setup", function (){
        wasm.aes_256_gcm_siv_key_iv_setup(key,iv);
    });
    key_iv_suite.add("rust_crypto_blowfish_key_iv_setup", function (){
        wasm.rust_crypto_blowfish_key_iv_setup(key);
    });
    key_iv_suite.on('complete', function() {
        for (let i = 0; i < this.length; i++) {
            console.log("Function " + this[i].name + " Avg time:" + this[i].stats.mean)
        }
        console.log('Fastest is ' + this.filter('fastest').map('name'));
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
        wasm.c2chacha20_encrypt_decrypt(text,key);

    });
    encrypt_decrypt_suite.add("blowfish_cbc_encrypt_decrypt", function (){
        wasm.blowfish_cbc_encrypt_decrypt(text,key);

    });
    encrypt_decrypt_suite.add("rust_crypto_aes_encrypt_decrypt", function (){
        wasm.rust_crypto_aes_encrypt_decrypt(text,key);

    });
    encrypt_decrypt_suite.add("aes_256_gcm_siv_encrypt_decrypt", function (){
        wasm.aes_256_gcm_siv_encrypt_decrypt(text,key,iv);

    });
    encrypt_decrypt_suite.add("rust_crypto_blowfish_encrypt_decrypt", function (){
        wasm.rust_crypto_blowfish_encrypt_decrypt(text,key);

    });
    encrypt_decrypt_suite.on('complete', function() {
        for (let i = 0; i < this.length; i++) {
            console.log("Function " + this[i].name + " Avg time:" + this[i].stats.mean)
        }
            //console.log('Fastest is ' + this.filter('fastest').map('name') + "ALL:" +  this[0].stats.mean);
    });
    // run async
    encrypt_decrypt_suite.run({ 'async': false });
}
const testing = () => {
    encrypt_decrypt_test();
    key_iv_setup_test();
}


$(document).ready(function() {
    //$("#runAlphabet").click(testAlphabet);
    $("#bench").click(testing);
});
