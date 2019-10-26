import * as wasm from "test-encryption";



const runTests = () => {
    var key = "aklsöjdfkölasdjfk";
    var text = document.getElementById("text1").value;
    var times = document.getElementById("number1").value;
    var times2 = document.getElementById("number2").value;
    var cryptos = ["c2chacha20_encrypt_decrypt", "blowfish_cbc_encrypt_decrypt", "rust_crypto_aes_encrypt_decrypt", "aes_256_gcm_siv_encrypt_decrypt", "rust_crypto_blowfish_encrypt_decrypt"];
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
    var table = "<table> <tr><th>Methode</th><th>avg time [ms]</th><th>max time [ms]</th><th>min time [ms]</th></tr>";
    for (let i = 0; i<cryptos.length;i++){
        var sum = 0;
        var array = runTimes.get(cryptos[i]);
        for( var j = 0; j < array.length; j++ ){
            sum += parseInt( array[i], 10 );
        }
        var avg = sum/array.length;
        var min = Math.min.apply(Math,array);
        var max = Math.max.apply(Math,array);
        table += "<tr><td>" + cryptos[i] + "</td><td> "+ avg +"</td><td> "+ max +"</td><td> "+ min +"</td></tr>";

    }
    table += "</table>";

    document.getElementById("tbl").innerHTML += table;
}
$(document).ready(function() {
    $("#run").click(runTests);
});
