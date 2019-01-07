<?php

declare(strict_types = 1);

namespace esnerda\Json2CsvProcessor;

use Keboola\Component\Config\BaseConfigDefinition;
use Symfony\Component\Config\Definition\Builder\ArrayNodeDefinition;

class ConfigDefinition extends BaseConfigDefinition {

    protected function getParametersDefinition(): ArrayNodeDefinition {
        $parametersNode = parent::getParametersDefinition();
        // @formatter:off
        /** @noinspection NullPointerExceptionInspection */
        $parametersNode
                ->children()
                ->variableNode('mapping')
                ->end()
                ->enumNode('placement')
                ->values(array('prepend', 'append'))
                ->end()
                ->scalarNode('append_row_nr')
                ->defaultValue(false)
                ->end()
                ->scalarNode('incremental')
                ->defaultValue(false)
                ->end()
                ->scalarNode('root_node')
                ->defaultValue(null)
                ->end()
                ->scalarNode('in_type')
                ->defaultValue('tables')
                ->end()
                ->variableNode('always_array')
                ->end()
                ->end()
        ;
        // @formatter:on
        return $parametersNode;
    }

}
