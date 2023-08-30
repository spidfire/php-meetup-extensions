<?php

// Open the file for writing
$file = fopen(__DIR__ . "/fake_log_10m.jsonl", "w");

for ($i = 0; $i < 10_000_000; $i++) {
    $timestamp = date('Y-m-d H:i:s', (int) gaussianRand(time(), 3600*24));
    $severity = ["INFO", "ERROR", "DEBUG"][rand(0, 2)];
    $message = "This is a {$severity} message";

    // Create the log entry as an associative array
    $logEntry = [
        "timestamp" => $timestamp,
        "severity" => $severity,
        "message" => $message
    ];

    // Convert the log entry to a JSON string
    $jsonLogEntry = json_encode($logEntry);

    // Write the JSON string to the file, followed by a newline
    fwrite($file, $jsonLogEntry . "\n");
}

// Close the file
fclose($file);


function gaussianRand($mean, $stdDev) {
    $u1 = rand(0, 32767) / 32767;
    $u2 = rand(0, 32767) / 32767;
    $randStdNormal = sqrt(-2 * log($u1)) * sin(2 * M_PI * $u2);
    return $mean + $stdDev * $randStdNormal;
}