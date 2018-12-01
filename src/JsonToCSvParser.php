<?php

/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

namespace esnerda\XML2CsvProcessor;

use Keboola\Json\Analyzer;
use Keboola\Json\Parser;
use Keboola\Json\Structure;
use Keboola\CsvMap\Mapper;

/**
 * Description of JsonToCSvParser
 *
 * @author esner
 */
class JsonToCSvParser {

    private $logger;
    private $parser;
    private $type;

    public function __construct($mapping, $logger, $type) {
        $this->logger = $logger;
        $this->type = $type;
        if ($mapping) {
            $this->parser = new Mapper($mapping, $type);
        } else {
            $this->parser = new Parser(new Analyzer($logger, new Structure(), true));
        }
    }

    public function parse($json_data) {
        $type = $this->getType($json_data);
        if (!is_array($json_data)) {
            $json_data = [$json_data];
        }
        if ($this->parser instanceof Mapper) {
            $this->parser->parse($json_data);
        } else {
            $this->parser->process($json_data, $type);
        }
    }

    public function getCsvFiles() {

        return $this->parser->getCsvFiles();
    }

    private function getType($json_data) {
        $type = key($json_data);
        return $type;
    }

    public function getResult() {
        return $this->parser->getCsvFiles();
    }

    private function addRowNumber($json) {
    }
}
