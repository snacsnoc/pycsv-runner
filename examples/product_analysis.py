import csv
import json

# Read CSV data from the placeholder
csv_data = """{{csv}}"""  

# Parse CSV
reader = csv.DictReader(csv_data.strip().splitlines())
data = list(reader)


for row in data:
    row['units_sold'] = int(row['units_sold'])
    row['unit_price'] = float(row['unit_price'])
    row['revenue'] = float(row['revenue'])

# Total revenue per category
category_revenue = {}
for row in data:
    category = row['category']
    revenue = row['revenue']
    category_revenue[category] = category_revenue.get(category, 0) + revenue

# Average units sold per product
product_units = {}
product_counts = {}
for row in data:
    product = row['product']
    units = row['units_sold']
    product_units[product] = product_units.get(product, 0) + units
    product_counts[product] = product_counts.get(product, 0) + 1

product_average_units = {
    product: product_units[product] / product_counts[product]
    for product in product_units
}

output = {
    "category_revenue": category_revenue,
    "product_average_units": product_average_units
}

# Output results as JSON
print(json.dumps(output))