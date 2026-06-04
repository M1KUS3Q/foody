## Foody - Refined Requirements

### Meal Management
(`foody meal`)

1. User can add a meal by name (`add Pasta`)
2. User can remove a meal by name (`remove Pasta`)
3. User can list all meals (`list`)
4. User can view a single meal (name + associated ingredients + assigned dayparts) (`view Pasta`)
5. User can rename a meal (`rename Pasta "Pasta Carbonara"`)

### Ingredient Management
(`foody ingredient`)

6. User can add an ingredient by name (`add tomato`)
7. User can remove an ingredient by name (`remove tomato`)
8. User can list all ingredients (`list`)
9. User can rename an ingredient (`rename tomato "cherry tomato"`)
10. User can view a single ingredient (name + how many meals use it, and which ones) (`view tomato`)
11. Removing an ingredient globally warns if it is associated with existing meals (`remove --force/-f tomato`)

**Meal–Ingredient Associations**
18. User can add an ingredient to a meal (`assign Pasta tomato`)
19. User can remove an ingredient from a meal (`unassign Pasta tomato`)
20. User can list all ingredients for a meal (via view meal, req. 4)

### Daypart Management
(`foody daypart`)

12. Default dayparts: `breakfast`, `lunch`, `dinner`
13. User can add a custom daypart by name (`add dessert`)
14. User can remove a daypart by name (warns if meals are assigned to it) (`remove dessert`)
15. User can list all dayparts (`list`)
16. User can assign one or more dayparts to a meal (`assign Pasta dessert,dinner`)
17. User can unassign a daypart from a meal (`unassign Pasta dessert`)
35. User can view a single daypart (name + which meals are assigned to it) 

### Meal Plan Management
(`foody plan`)

21. User can create a named meal plan (`add Plan1`)
22. User can delete a meal plan (`remove Plan1`)
23. User can list all meal plans (`list`)
24. User can view a full meal plan (`view Plan1`)
25. A meal plan is structured as a sequence of **named days** (e.g. Monday–Sunday, or Day 1–Day 5) — not tied to calendar dates
26. Each day contains one slot per daypart; slots can be empty (`null`) to represent skipped meals (takeout, fasting, etc.)
27. User can assign a meal to a plan/day/daypart slot (`assign Plan1 Monday Dinner Pasta`)
28. User can clear a slot (set to null) in a plan (`assign Plan1 Monday Dinner null`)
29. Assigning a meal to a daypart slot it is not tagged with produces a warning but is not blocked

### Grocery List Generation
(`foody grocery`)

30. User can generate a grocery list from a meal plan (`plan Plan1`)
31. Grocery list shows each ingredient, how many meals in the plan require it, and which meals those are
32. User can export the grocery list to plain-text or CSV (`--export data.csv/data.txt`)
33. User can display the grocery list in the terminal (default)
34. User can display a grocery list for a single meal (`meal Pasta`)