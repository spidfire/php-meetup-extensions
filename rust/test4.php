<?php


// resize(__DIR__ . '/../shepherd.jpg', __DIR__ . '/../shepherd-cropped.jpg');

$bench = microtime(true);


$photo = new ImageResize(__DIR__ . '/../shepherd.jpg');

$aspectRatio = $photo->x / $photo->y;
$photo->crop((int) (200*$aspectRatio), 200);

$photo->save(__DIR__ . '/../shepherd-cropped.jpg');


echo "Done: " . (microtime(true) - $bench) . "\n";
