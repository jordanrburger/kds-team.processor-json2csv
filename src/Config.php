<?php

declare(strict_types = 1);

namespace esnerda\XML2CsvProcessor;

use Keboola\Component\Config\BaseConfig;

class Config extends BaseConfig {

    // @todo implement your custom getters
    public function getMapping(): array {
        return $this->getValue(['parameters', 'mapping']);
    }

    public function getAppendRowNr(): bool {
        return $this->getValue(['parameters', 'append_row_nr']);
    }

    public function isIncremental(): bool {
        return $this->getValue(['parameters', 'incremental']);
    }

    public function getRootNode(): string {
        return $this->getValue(['parameters', 'root_node']);
    }

    public function getInputType(): string {
        return $this->getValue(['parameters', 'in_type']);
    }

    public function getForceArrayAttributes(): array {
        return $this->getValue(['parameters', 'always_array']);
    }

}
