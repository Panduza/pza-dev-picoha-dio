
const { exec } = require('child_process');
const fs = require('fs');
const express = require('express');

function launch_campagne(tag) {
    // TODO : find a way to execute 'robot -i tag .' on the server
    console.log('launch_campagne : robot', tag, '.')
}


function launch_campagne_wrapper(id) {
    console.log('launch_campagne_wrapper')
    if (campagne_running === 0){
        // Locked commend sender
        campagne_running=1;
        // Change color indicator 
        var property = document.getElementById(id);
        property.style.backgroundColor = "gray";
        // Get campagne var
        const tag = document.getElementById('tag_select').value;
        // Launch campagne 
        launch_campagne(tag);
    }
    else {
        console.log('Command running')
    }
}
