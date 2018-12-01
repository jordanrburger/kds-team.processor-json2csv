

# JSON2CSV processor
Keboola Connection processor for JSON to CSV conversion.


Converts JSON files to CSV. 

**Credits:**
- For JSON2CSV conversion uses Keboola developed [Json parser](https://github.com/keboola/php-jsonparser) and [CsvMap](https://github.com/keboola/php-csvmap) for analysis and automatic conversion from JSON to CSV. Supports Generic Ex -like mapping configuration.

# Usage
## Configuration parameters

- **in_type** (enum [`files`,`tables`]) -  specifies the input folder where to look for input data. e.g. when set to `table` the processor will look for inpu in `/in/tables/` folder.
- **incremental** (bool) - flag whether the resulting tables should be uploaded incrementally. Makes most sense with mapping setup, since it allows you to specify primary keys.
- **root_node** (string) - `.` separated path to the root node of the resulting JSON - usually you only want to map the root array, not all the wrapper tags. For more info see examples below.

## Examples

### JSON example
```json
{
	"root_el": {
		"orders": {
			"order": [{
					"id": "1",
					"date": "2018-01-01",
					"cust_name": "David",
					"order-item": [{
							"price": {
								"xml_attr_currency": "CZK",
								"txt_content_": "100"
							},
							"item": "Umbrella",
							"row_nr": 1
						}, {
							"price": {
								"xml_attr_currency": "CZK",
								"txt_content_": "200"
							},
							"item": "Rain Coat",
							"row_nr": 2
						}
					],
					"row_nr": 1
				}, {
					"id": "2",
					"date": "2018-07-02",
					"cust_name": "Tom",
					"order-item": {
						"price": {
							"xml_attr_currency": "GBP",
							"txt_content_": "100"
						},
						"item": "Sun Screen",
						"row_nr": 1
					},
					"row_nr": 2
				}
			]
		}
	}
}
```

### Simple Example
Assuming JSON file in `/in/files/`.
#### Configuration
```json
{
    "definition": {
        "component": "kds-team.processor-json2csv"
    },
    "parameters" : {
	"mapping" : {},
	"incremental":true,
	"root_node" : "",
    "in_type": "files"
	}
}
```
The above produces two tables according to mapping setting `order.csv`:

| root_el_orders_order |
|--|
| root_el.root_el.orders_a91b89e33c2b324f4204686aa64a0d5f |


and `root_el_root_el_orders_order_order-item.csv`:

| price_xml_attr_currency | price_txt_content | item | row_nr| JSON_parentId
|--|--|--|--|--|
| CZK | 100| Umbrella |1|root_el.root_el.orders.order_d3859e7943e09800b982215f5c4434c6
| CZK | 200| Rain Coat|2|root_el.root_el.orders.order_d3859e7943e09800b982215f5c4434c6
| GBP | 100| Sun Screen|1|root_el.root_el.orders.order_d3859e7943e09800b982215f5c4434c6




### Advanced Example 1 - nested arrays, with mapping
Assuming JSON file in `/in/files/`.
#### Configuration
```json
{
    "definition": {
        "component": "kds-team.processor-xml2csv"
    },
    "parameters" : {
	"mapping" : {
			"id": {
				"type": "column",
				"mapping": {
					"destination": "order_id",
					"primaryKey": true
				}
			},
			"date": {
				"type": "column",
				"mapping": {
					"destination": "order_date"
				}
			},
			"cust_name": {
				"type": "column",
				"mapping": {
					"destination": "customer_name"
				}
			},
			"order-item": {
				"type": "table",
				"destination": "order-items",
				"parentKey": {
					"primaryKey": true,
					"destination": "order_id"
				},
				"tableMapping": {
					"row_nr": {
						"type": "column",
						"mapping": {
							"destination": "row_nr",

							"primaryKey": true
						}
					},
					"price.xml_attr_currency": {
						"type": "column",
						"mapping": {
							"destination": "currency"
						}
					},
					"price.txt_content_": {
						"type": "column",
						"mapping": {
							"destination": "price_value"
						}
					},
					"item": {
						"type": "column",
						"mapping": {
							"destination": "item_name"
						}
					}
				}
			}},		
	"incremental":true,
	"root_node" : "root_el.orders.order",
    "in_type": "files"
	}
}
```

#### Intermediate converted JSON - effect of `root_node` parameter
```json
[{
		"id": "1",
		"date": "2018-01-01",
		"cust_name": "David",
		"order-item": [{
				"price": {
					"xml_attr_currency": "CZK",
					"txt_content_": "100"
				},
				"item": "Umbrella",
				"row_nr": 1
			}, {
				"price": {
					"xml_attr_currency": "CZK",
					"txt_content_": "200"
				},
				"item": "Rain Coat",
				"row_nr": 2
			}
		],
		"row_nr": 1
	}, {
		"id": "2",
		"date": "2018-07-02",
		"cust_name": "Tom",
		"order-item": {
			"price": {
				"xml_attr_currency": "GBP",
				"txt_content_": "100"
			},
			"item": "Sun Screen",
			"row_nr": 1
		},
		"row_nr": 2
	}
]
```

The above produces two tables  according to mapping setting `order.csv`:

| order_id | order_date | customer_name |
|--|--|--|
| 1 |  2018-01-01| David |
| 2 |  2018-01-02|  Tom|

and `order-items.csv`:

| row_nr | currency | price_value | item_name| order_id
|--|--|--|--|--|
| 1 | CZK| 100 |Umbrella|1
| 2 | CZK| 200|Rain Coat|2
| 1 | GBP| 100|Sun Screen|2




For more information about Generic mapping plese refer to [the generic ex documentation](https://developers.keboola.com/extend/generic-extractor/map/)




For more information about processors, please refer to [the developers documentation](https://developers.keboola.com/extend/component/processors/).
