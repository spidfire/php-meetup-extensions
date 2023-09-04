<?php


// resize(__DIR__ . '/../shepherd.jpg', __DIR__ . '/../shepherd-cropped.jpg');

$bench = microtime(true);


$photo = new ImageResize(__DIR__ . '/../shepherd.jpg');
$photo->crop(200, 200);
$photo->save(__DIR__ . '/../shepherd-cropped.jpg');


echo "Done: " . (microtime(true) - $bench) . "\n";