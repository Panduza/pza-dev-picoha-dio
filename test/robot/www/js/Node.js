const express = require('express');
const fs = require('fs');
const app = express();

app.use(express.static('public'));

app.get('/output', (req, res) => {
    fs.readFile('output.log', 'utf8', (err, data) => {
        if (err) {
            res.status(500).send('Erreur de lecture du fichier');
            return;
        }
        res.send(data);
    });
});

app.listen(3000, () => {
    console.log('Serveur en écoute sur le port 3000');
});
