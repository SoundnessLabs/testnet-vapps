#!/usr/bin/env node

const fs = require('fs');

function validate(file) {
    if (!fs.existsSync(file)) return false;
    
    const content = fs.readFileSync(file, 'utf8');
    const required = ['github_username:', 'discord_id:', 'project name', 'category'];
    
    return required.every(field => 
        content.toLowerCase().includes(field.toLowerCase())
    );
}

if (require.main === module) {
    const file = process.argv[2];
    if (!file) {
        console.error('Usage: node validate.js <file>');
        process.exit(1);
    }
    
    if (validate(file)) {
        console.log('✅ Valid');
        process.exit(0);
    } else {
        console.log('❌ Invalid');
        process.exit(1);
    }
}
