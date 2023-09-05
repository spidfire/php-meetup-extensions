<?php



function json_aggregate_count_occurrences_php($filename, $key) {
    $file = fopen($filename, "r");
    $results = [];
    while (($line = fgets($file)) !== false) {
        $json = json_decode($line, true);
        $results[$json[$key]] = ($results[$json[$key]] ?? 0) + 1;
    }
    fclose($file);
    return $results;
}

$testfile = __DIR__ . "/../fake_log_10m.jsonl";

$start = microtime(true);
$results = json_aggregate_count_occurrences($testfile, "severity");
var_dump($results);
echo "RUST: " . (microtime(true) - $start) . "\n";


$start = microtime(true);
$result = json_aggregate_count_occurrences_php($testfile, "severity");
var_dump($result);
echo "PHP: " . (microtime(true) - $start) . "\n";


