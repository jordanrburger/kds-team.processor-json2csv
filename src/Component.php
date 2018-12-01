<?php

declare(strict_types = 1);

namespace esnerda\XML2CsvProcessor;

use Keboola\Component\BaseComponent;

class Component extends BaseComponent {

    public function run(): void {
        $type = 'root';
        if ($this->getConfig()->getRootNode() != NULL) {
            $nodes = explode('.', $this->getConfig()->getRootNode());
            $type = $nodes[count($nodes) - 1];
        }
        $jsonParser = new JsonToCSvParser($this->getConfig()->getMapping(), $this->getLogger(), $type);

        $processor = new Processor(
                $jsonParser, $this->getConfig()->getAppendRowNr(), 
                $this->getConfig()->getForceArrayAttributes(), 
                $this->getConfig()->isIncremental(), 
                $this->getConfig()->getRootNode(),
                $this->getLogger()
        );



        $processor->convert($this->getDataDir(), $this->getConfig()->getInputType());
    }

    protected function getConfigClass(): string {
        return Config::class;
    }

    protected function getConfigDefinitionClass(): string {
        return ConfigDefinition::class;
    }

}
