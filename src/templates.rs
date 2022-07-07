pub fn index() -> &'static str {
    "
<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <title>LooneyTube</title>
    <link href=\"/c/css/bootstrap.min.css\" rel=\"stylesheet\">
    <link href=\"/c/css/bootstrap-grid.min.css\" rel=\"stylesheet\">
</head>
<body>

<div class=\"container\">
    <h1><a href=\"/\" style=\"margin: 2rem; display: inline-block; width: 100%; text-align: center;\"><img style=\"height: 50px; max-width: 50%;\" src=\"/c/img/logo.png\" alt=\"LooneyTube\" /></a></h1>
    <div class=\"row\">
        {{#each categories}}
            <div class=\"col-6 col-md-3\">
                <a style=\"color: black; text-decoration: none;\" href=\"/{{ this.slug }}\" class=\"card\">
                    <img src=\"{{ this.picture }}\" class=\"card-img-top\" alt=\"{{ this.name }}\">
                    <div class=\"card-body\">
                        <h5 class=\"card-title\">{{ this.name }}</h5>
                    </div>
                </a>
            </div>
        {{/each}}
    </div>
</div>

<script src=\"/c/js/bootstrap.bundle.min.js\"></script>
</body>
</html>
    "
}


pub fn category() -> &'static str {
    "
<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <title>LooneyTube</title>
    <link href=\"/c/css/bootstrap.min.css\" rel=\"stylesheet\">
    <link href=\"/c/css/bootstrap-grid.min.css\" rel=\"stylesheet\">
</head>
<body>

<style>
#video {
    display:none;
    z-index: 1000;
}

.overlay {
    background-color: #000;
}

.overlay #video {
    display: block;
}

.overlay #container {
    display: none;
}
</style>

<video class=\"hidden\" id=\"video\" style=\"position: absolute; top: 0; left: 0; width: 100%; height: 100%; margin: 0; padding: 0;\" autoplay=\"autoplay\" controls=\"controls\"></video>

<div id=\"container\" class=\"container\">
    <h1><a href=\"/\" style=\"margin: 2rem; display: inline-block; width: 100%; text-align: center;\"><img style=\"height: 50px; max-width: 50%;\" src=\"/c/img/logo.png\" alt=\"LooneyTube\" /></a></h1>
    <h2 style=\"display: inline-block; width: 100%; text-align: center\">{{ category.name }}</h2>

    <div class=\"row\">
        {{#each videos}}
            <div class=\"col-6 col-md-3\">
                <a style=\"color: black; text-decoration: none;\" href=\"{{ this.path }}\" class=\"video-link card\">
                    <img src=\"{{ this.picture }}\" class=\"card-img-top\" alt=\"{{ this.name }}\">
                    <div class=\"card-body\">
                        <h5 class=\"card-title\">{{ this.name }}</h5>
                    </div>
                </a>
        </div>
        {{/each}}
    </div>
</div>

<script src=\"/c/js/bootstrap.bundle.min.js\"></script>

<script type=\"text/javascript\">
const links = document.getElementsByClassName('video-link');

for (const link of links) {
    link.addEventListener('click', function(e) {
        e.preventDefault();
        const video = document.getElementById('video');
        const body = document.getElementsByTagName('body')[0];

        video.src = link.href;
        video.classList.remove('hidden');
        body.classList.add('overlay');
    });

    document.getElementById('video').onended = function () {
        window.location.href = '/';
    };
}
</script>
</body>
</html>
    "
}
