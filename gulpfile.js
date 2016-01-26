// Load plugins.
var gulp = require("gulp");
var concat = require("gulp-concat");
var uglify = require("gulp-uglify");
var del = require("del");
var bower = require("gulp-bower");
var less = require("gulp-less");
var nano = require("gulp-cssnano");

var config = {
    vendorsrc: [
        "bower_components/jquery/jquery.js",
        "bower_components/bootstrap/dist/js/bootstrap.js"
    ],
    vendorbundle: "public/javascript/vendor-bundle.min.js",
    lesssrc: "app/assets/stylesheets/application.less",
    cssdest: "public/stylesheets/application.css"
}

// Restore all bower packages.
gulp.task("bower-restore", function() {
    return bower();
});

// Synchronously delete the output script files.
gulp.task("clean-vendor-bundle", function(cb) {
    return del([config.vendorbundle], cb);
});

// Create bundled file of vendor scripts.
gulp.task("vendor-bundle", ["clean-vendor-bundle", "bower-restore"], function() {
    return gulp.src(config.vendorsrc)
               .pipe(uglify())
               .pipe(concat("vendor-bundle.min.js"))
               .pipe(gulp.dest("public/javascript"));
});

// Synchronously delete the output style files.
gulp.task("clean-styles", function(cb) {
    return del([config.cssdest], cb);
});

// Compiles source less files into css file.
gulp.task("less", ["clean-styles", "bower-restore"], function() {
    return gulp.src(config.lesssrc)
               .pipe(less())
               .pipe(nano())
               .pipe(concat("application.min.css"))
               .pipe(gulp.dest("public/stylesheets"))
});

// Copies fonts to public directory.
gulp.task("fonts", ["clean-styles", "bower-restore"], function() {
    
});

// Copies images to public directory.
gulp.task("images", ["clean-styles", "bower-restore"], function() {
    
});

// Generic styles and assets task.
gulp.task("styles", ["less", "fonts", "images"], function() {
    
});

// Watch for changes in assets directory and rerun tasks if necessary.
gulp.task("watch", function() {
    return gulp.watch(["app/**/*.js", "app/**/*.less"], ["less"]);
});

// Set a default task.
gulp.task("default", ["vendor-bundle", "styles"], function() { });
