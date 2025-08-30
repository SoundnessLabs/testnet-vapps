#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * Validate file content for required fields.
 * @param {string} file - Path to the file.
 * @param {string[]} [required] - List of required fields.
 * @returns {{valid: boolean, missing: string[]}}
 */
function validate(file, required = ['github_username:', 'discord_id:', 'project name', 'category']) {
    if (!fs.existsSync(file)) return { valid: false, missing: required };

    const content = fs.readFileSync(file, 'utf8').toLowerCase();
    const missing = required.filter(field => !content.includes(field.toLowerCase()));
    return { valid: missing.length === 0, missing };
}

/**
 * Validate all files in a directory.
 * @param {string} dir - Path to directory.
 * @param {string[]} required - List of required fields.
 */
function validateDirectory(dir, required) {
    const files = fs.readdirSync(dir);
    let allValid = true;
    files.forEach(file => {
        const filePath = path.join(dir, file);
        if (fs.lstatSync(filePath).isFile()) {
            const result = validate(filePath, required);
            if (result.valid) {
                console.log(`✅ ${file}: Valid`);
            } else {
                allValid = false;
                console.log(`❌ ${file}: Missing fields - ${result.missing.join(', ')}`);
            }
        }
    });
    process.exit(allValid ? 0 : 1);
}

// CLI
if (require.main === module) {
    const target = process.argv[2];
    if (!target) {
        console.error('Usage: node validate.js <file|directory>');
        process.exit(1);
    }
    const required = ['github_username:', 'discord_id:', 'project name', 'category'];
    if (fs.lstatSync(target).isDirectory()) {
        validateDirectory(target, required);
    } else {
        const result = validate(target, required);
        if (result.valid) {
            console.log('✅ Valid');
            process.exit(0);
        } else {
            console.log(`❌ Invalid. Missing fields: ${result.missing.join(', ')}`);
            process.exit(1);
        }
    }
}
module.exports = { validate };