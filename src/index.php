<?php

require '../vendor/autoload.php';

use esnerda\XML2CsvProcessor\XML2JsonConverter;
use Keboola\CsvTable\Table;


use Keboola\Json\Analyzer;
use Keboola\Json\Parser;
use Keboola\Json\Structure;
use Keboola\CsvMap\Mapper;
use Symfony\Component\Filesystem\Filesystem;
use Keboola\Component\Logger;

ini_set('memory_limit','1024M');
$memory_limit = ini_get('memory_limit');

//$file = file_get_contents('C:\Users\esner\Documents\Prace\KBC\xmltocsv\sample-small.json');
$file = file_get_contents('C:\Users\esner\Documents\Prace\KBC\xmltocsv\data\out\tables\test.json');
$testfolder = 'C:\Users\esner\Documents\Prace\KBC\xmltocsv';

$parser = new Parser(new Analyzer(new Logger(), new Structure()));
$json = json_decode($file);
$parser->process([$json]);

$results = $parser->getCsvFiles();
$fs = new Filesystem();
foreach ($results as $res){
    $dest = $testfolder.'/'.$res->getName().'.csv';
     copy($res->getPathname(), $dest);
}

//sample files
$sample1 = 'C:\Users\esner\Documents\Prace\KBC\xmltocsv\sample.xml';
$sample2 = 'C:\Users\esner\Documents\Prace\KBC\xmltocsv\sample2.xml';
$parser = new XML2JsonConverter();

$sample1String = file_get_contents($sample1);

echo $parser->xml2json($sample1String, true);