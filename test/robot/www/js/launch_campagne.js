const { exec } = require('child_process');
const fs = require('fs');

function launch_campagne(tag) {
    const command = `robot . ${tag}`;
    exec(command, (error, stdout, stderr) => {
        if (error) {
            console.error(`Erreur: ${error.message}`);
            fs.appendFileSync('output.log', `Erreur: ${error.message}\n`);
            return;
        }
        if (stderr) {
            console.error(`Erreur: ${stderr}`);
            fs.appendFileSync('output.log', `Erreur: ${stderr}\n`);
            return;
        }
        console.log(`Résultat: ${stdout}`);
        fs.appendFileSync('output.log', `Résultat: ${stdout}\n`);
    });
}