const uglify = require('uglify-es');
const minifier =  require('html-minifier');
const pp = require('preprocess');
const sass = require('node-sass');

const fs = require('fs');
const fse = require('fs-extra');
const async = require('async');

// Where all the pre-prod source files coming from
const inputDir = './src'
// Where all the prod-ready dist files going
const outputDir = './dist';

// Where the dev JS files live
const jsInputRoot = `${inputDir}/js`;
// Where the concatenated, uglified JS file will live
const jsOutputFile = 'mashed.js';
const jsOutput = `${outputDir}/${jsOutputFile}`;

// Where the dev HTML index file lives
const htmlInput = `${inputDir}/index.html`;
// Where the processed, minified HTML index file will live
const htmlOutput = `${outputDir}/index.html`;

// Where the uncompiled SASS style file lives
const styleRoot = `${inputDir}/style`;
const scssInput = `${styleRoot}/style.scss`;
// Where the compiled CSS file will live
const cssOutputFile = 'style.css'
const cssOutput = `${outputDir}/${cssOutputFile}`;

const blog = 'entries';
const blogInput = `${inputDir}/${blog}`
const blogOutput = `${outputDir}/${blog}`;

const img = 'img';
const imgInput = `${inputDir}/${img}`;
const imgOutput = `${outputDir}/${img}`;

// Used with preprocess. Handles replacing pre-prod link and 
// script references with the prod versions. 
const htmlPreprocessOptions = {
    NODE_ENV: 'prod',
    MIN_STYLE: cssOutputFile,
    MIN_JS: jsOutputFile
}

// Used with html-minify
const htmlMinificationOptions = {
    removeComments: true,
    collapseWhitespace: true
}

// ============================
//      Start building!
// ============================
console.log('Preparing output dir: ', outputDir);
prepareOutputDirectory(outputDir);

console.log('Squashing JavaScript files to %s', jsOutput);
concatContent(jsInputRoot, 'js', jsOutput, concatJavascript);

console.log('Minifying HTML page to %s', htmlOutput)
preprocessAndMinifyHTML(htmlInput, htmlOutput, htmlPreprocessOptions, htmlMinificationOptions);

console.log('Compiling SASS to %s', cssOutput);
compileSASS(scssInput, cssOutput);

console.log('Copying blog entries to %s', blogOutput);
copyFiles(blogInput, blogOutput);

console.log('Copying images to %s', imgOutput);
copyFiles(imgInput, imgOutput);

console.log('DONE');
// ============================
//        End building!
// ============================

/**
 * Creates an output directory if one does not exist.
 * @param {string} outDir
 *      Output directory path.
 */
function prepareOutputDirectory(outDir) {
    if (!fs.existsSync(outDir)) {
        // The output directory doesn't exist yet, so make it.
        fs.mkdirSync(outDir);
    } else {
        if (!fs.statSync(outDir).isDirectory()) {
            // The provided path already exists and it is not a directory.
            console.error('%s is not a directory.', outDir);
            return;
        } else {
            // Remove all directories and files currently contained in the output directory.
            fs.readdirSync(outDir)
                .map((file) => outDir + "/" + file)
                .forEach((path) => {
                    fse.removeSync(path);
                });
        }
    } 
}

/**
 * Concatenates files of a specified type using an optional concatenation algorithm.
 * @param {*} inDir 
 *      Directory containing files to concatenate.
 * @param {*} ext 
 *      Extension of files to concatenate.
 * @param {*} outFile 
 *      Desination file of the concatenated content.
 * @param {(string[], string[]) => ConcatResult} concat 
 *      Optional concatenation function which expects an array of paths
 *      and a corresponding array of contents as inputs.  Returns the 
 *      concatenated result.
 */
function concatContent(inDir, ext, outFile, concat) {
    // combine the javascript files
    new Promise((resolve, reject) => {
        var paths;
        async.map(paths=getAllFiles(inDir, ext), fs.readFile, (err, results) => {            
            if (err)
                return reject(err);

            var result;
            if (concat) {
                result = concat(paths, results);
            } else {
                result = results.join("\n");
            }

            if (result.error)
                reject(result.error);

            resolve(result.content);
        })
    }).then((value) => {
        fs.writeFileSync(jsOutput, value);
    }).catch((err) => {
        console.log(err);
    });
}

/**
 * Recursively retreives the paths of all files with the given extension
 * contained with the provided root directory.
 * @param {string} rootDir 
 *      The directory to search for matching files.
 * @param {string} ext 
 *      The extension of files to search for.
 * @returns {string[]} a list of all files with the provided extension.
 */
function getAllFiles(rootDir, ext) {
    var matchingFiles = [];
    
    // Start with the root files.
    var files = (fs.readdirSync(rootDir) || []).map((file) => rootDir + "/" + file);
    
    while(files.length > 0) {
        files = files.reduce((acc, path) => {
            let st = fs.statSync(path); 
            if(st.isDirectory()) {
                // If the path is a directory, accumulate its contents.
                acc.push(...fs.readdirSync(path).map((file) => path + "/" + file));
            } else if (st.isFile() && path.match(`/*\.${ext}/`)) {
                // If the path is a matching file, record it.
                matchingFiles.push(path);
            }
            return acc;
        }, []);
    }

    return matchingFiles;
}

/**
 * Describes result of concatenation operation.
 */
class ConcatResult {
    constructor(error, content) {
        this.error = error;
        this.content = content;
    }
}

/**
 * Concatenates JS files using uglify-es.
 * @param {string[]} paths 
 *      Paths of the files to concatenate.
 * @param {stirng[]} contents 
 *      Contents of the files to concatenate.
 * @returns {ConcatResult} describing the result of 
 *      the operation.
 */
function concatJavascript(paths, contents) {
    let dict = {};
    contents.forEach((content, i) => {
        dict[paths[i]] = content.toString();
    });

    let result = uglify.minify(dict);

    return new ConcatResult(result.error, result.code);
}

/**
 * Uses preprocess and html-minifier to preprocess and minify input HTML respectively.
 * @param {string} inputFile 
 *      The path of the input HTML file.
 * @param {string} outputFile 
 *      The pat of the output HTML file.
 * @param {*} ppOptions 
 *      Preprocess options.
 * @param {*} minifyOptions
 *      html-minify options. 
 */
function preprocessAndMinifyHTML(inputFile, outputFile, ppOptions, minifyOptions) {
    // Read contents of the HTML file.
    let input = fs.readFileSync(inputFile).toString();

    // Replace preprocess directives.
    let processedInput = pp.preprocess(input, ppOptions);

    // Minify the processed input content.
    let minifiedInput = minifier.minify(processedInput, minifyOptions); 

    // Write the minifed, processed input content to the output file.
    fs.writeFileSync(outputFile, minifiedInput);
}

/**
 * Compiles SASS to CSS.
 * @param {*} inputFile 
 *      The SASS file to compile.
 * @param {*} outputFile 
 *      The output CSS file.
 */
function compileSASS(inputFile, outputFile) {
    sass.render({
        file: inputFile,
        outFile: outputFile
    }, (err, result) => {
        if (!err) {
            fs.writeFileSync(outputFile, result.css);
        }
    });
}

function copyFiles(input, output) {
    try {
        fse.copySync(input, output)
    } catch (err) {
        console.error(err)
    }
}