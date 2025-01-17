<?php

declare(strict_types = 1);

namespace esnerda\Json2CsvProcessor;

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

    public function addFileName(): bool {
        return $this->getValue(['parameters', 'add_file_name']);
    }

    public function getRootNode(): string {
        return $this->getValue(['parameters', 'root_node']);
    }

    public function getInputType(): string {
        return $this->getValue(['parameters', 'in_type']);
    }

}
