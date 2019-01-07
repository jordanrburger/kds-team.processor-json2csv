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


//order-item is array
$json1 = json_decode('{
	"root_el": {
		"orders": {
			"order": {
				"id": "1",
				"date": "2018-01-01",
				"cust_name": "David",
				"order-item": [{
						"price": {
							"xml_attr_currency": "CZK",
							"txt_content_": "100"
						},
						"item": "Umbrella"
					}, {
						"price": {
							"xml_attr_currency": "CZK",
							"txt_content_": "200"
						},
						"item": "Rain Coat"
					}
				]
			}
		}
	}
}
');
// slightly different structure - order-item is object

$json2 = json_decode('{
	"root_el": {
		"orders": {
			"order": {
				"id": "2",
				"date": "2018-07-02",
				"cust_name": "Tom",
				"order-item": {
					"price": {
						"xml_attr_currency": "GBP",
						"txt_content_": "100"
					},
					"item": "Sun Screen"
				}
			}
		}
	}
}
');
$testfolder = '../data/out/tables';

$parser = new Parser(new Analyzer(new Logger(), new Structure()));

$parser->process([$json1]);
$parser->process([$json2]);

$results = $parser->getCsvFiles();
$fs = new Filesystem();
foreach ($results as $res){
    $dest = $testfolder.'/'.$res->getName().'.csv';
     copy($res->getPathname(), $dest);
}
