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

<div class=\"container\">
    <h1><a href=\"/\" style=\"margin: 2rem; display: inline-block; width: 100%; text-align: center;\"><img style=\"height: 50px; max-width: 50%;\" src=\"/c/img/logo.png\" alt=\"LooneyTube\" /></a></h1>
    <h2 style=\"display: inline-block; width: 100%; text-align: center\">{{ category.name }}</h2>

<div class=\"row\">
    {{#each videos}}
        <div class=\"col-6 col-md-3\">
        <a style=\"color: black; text-decoration: none;\" href=\"/{{ this.full_slug }}\" class=\"card\">
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

pub fn video() -> &'static str {
    "
<!doctype html>
<html lang=\"en\" style=\"width: 100%; height: 100%; margin: 0; padding: 0;\">
<head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <title>LooneyTube</title>
</head>
<body style=\"width: 100%; height: 100%; margin: 0; padding: 0; background: black;\">
<video id=\"video\" style=\"width: 100%; height: 100%; margin: 0; padding: 0;\" src=\"{{ video.path }}\" autoplay=\"autoplay\" controls=\"controls\">
    <source src=\"{{ video.path }}\" type=\"video/mp4\"/>
</video>

<script type=\"application/javascript\">
    document.getElementById('video').onended = function () {
        window.location.href = '/';
    };
</script>
</body>
</html>
    "
}
