# pgkv

> Postgres extension to create a KV Store in Postgres DB

### Usage

- To set a value for a key

```sql
select set('first', '{"hello":"world"}');
```

- To get the value for a set key

```sql
select get('first');
```

- To delete the value for a set key

```sql
select del('first');
```
