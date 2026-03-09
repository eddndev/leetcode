---
title: "0273 Integer to English Words - ES"
problemUrl: "https://leetcode.com/problems/integer-to-english-words/"
difficulty: "Hard"
pubDate: "2026-03-08"
tags: ["string", "recursion", "math"]
complexity:
  time: "O(1) ya que la entrada esta acotada por 2^31 - 1 (como maximo 10 digitos)"
  space: "O(1) por la misma razon"
---

# Ensenarle a una Maquina a Leer Numeros en Voz Alta

## El Problema
Dado un entero no negativo `num`, convertirlo a su representacion en palabras en ingles. La entrada esta garantizada a ser menor que 2^31 - 1 (como maximo 2,147,483,647). Por ejemplo, `1234567` se convierte en `"One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"`, y `0` se convierte en `"Zero"`.

## Por Que Este Problema Es Enganosamente Complicado

A primera vista, parece un simple ejercicio de mapeo. Solo buscar cada digito y concatenar palabras, verdad? Pero el idioma ingles no funciona asi. Los numeros no se leen digito por digito -- se leen en grupos de tres con palabras de escala (Thousand, Million, Billion) separando los grupos. Dentro de cada grupo, las reglas cambian segun la magnitud: las centenas tienen su propia palabra, los numeros del 11 al 19 son irregulares (Eleven, Twelve, Thirteen...), y las decenas siguen otro patron mas (Twenty, Thirty, Forty...). Una serie de condicionales por fuerza bruta se convertiria en un desastre ilegible.

El verdadero desafio es encontrar una descomposicion limpia que maneje todos estos casos sin duplicacion.

## La Estrategia: Dividir por Miles, Conquistar por Recursion

### Dividir en Grupos de Tres

Me di cuenta de que el ingles lee los numeros grandes en grupos de tres digitos, separados por palabras de escala. Tomemos `2,147,483,647`: se lee como "Two Billion" + "One Hundred Forty Seven Million" + "Four Hundred Eighty Three Thousand" + "Six Hundred Forty Seven". Cada grupo de tres digitos sigue exactamente las mismas reglas -- es simplemente un numero del 0 al 999. La unica diferencia es cual palabra de escala lo acompana.

Asi que el bucle externo es simple: extraer repetidamente los ultimos tres digitos con `n % 1000`, convertirlos a palabras usando un helper, agregar la palabra de escala apropiada (Thousand, Million o Billion), y luego desplazar `n` a la derecha dividiendo entre 1000.

### El Helper: Convirtiendo 0-999

La funcion helper maneja numeros del 0 al 999 usando tres niveles de logica:

1. **Caso base (0)**: devolver una cadena vacia, porque un grupo de ceros no deberia producir ninguna palabra (no decimos "Zero Thousand").

2. **Menor que 20**: busqueda directa en un arreglo. Esto cubre las palabras unicas de One a Nineteen. Los numeros del 11 al 19 son irregulares en ingles, asi que no hay atajo -- deben listarse explicitamente.

3. **20 a 99**: la palabra de la decena (Twenty, Thirty, ..., Ninety) mas una llamada recursiva para el digito de las unidades.

4. **100 a 999**: la palabra de las centenas mas "Hundred" mas una llamada recursiva para el resto (los ultimos dos digitos).

La recursion es superficial -- como maximo dos niveles de profundidad -- asi que no hay riesgo de desbordamiento de pila ni problemas de rendimiento.

### Manejo de Espacios

Antepongo un espacio antes de cada palabra en los arreglos de busqueda (por ejemplo, `" One"`, `" Two"`). Esto significa que cada fragmento comienza naturalmente con un espacio, y el resultado final solo necesita un `.trim()` para eliminar el espacio inicial sobrante. Este enfoque evita la logica engorrosa de insertar espacios condicionalmente entre palabras.

### Un Recorrido Paso a Paso

Para `num = 1234567891`:

```
Iteracion 1: n % 1000 = 891.  helper(891) = " Eight Hundred Ninety One"
  Escala: "" (grupo de unidades). Resultado parcial: " Eight Hundred Ninety One"

Iteracion 2: n % 1000 = 567.  helper(567) = " Five Hundred Sixty Seven"
  Escala: " Thousand". Resultado parcial: " Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

Iteracion 3: n % 1000 = 234.  helper(234) = " Two Hundred Thirty Four"
  Escala: " Million". Resultado parcial: " Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

Iteracion 4: n % 1000 = 1.    helper(1) = " One"
  Escala: " Billion". Resultado parcial: " One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"

Despues de trim: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
```

Observa como `n % 1000 != 0` protege contra los grupos de ceros. Si la entrada fuera `1000000`, el segundo y tercer grupo (ambos 000) se omiten por completo, produciendo solo `"One Million"` sin un "Thousand" espurio ni espacios sobrantes.

## Solucion en Rust

```rust
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let thousands = ["", " Thousand", " Million", " Billion"];

        let mut res = String::new();
        let mut n = num;
        let mut i = 0;

        while n > 0 {
            if n % 1000 != 0 {
                res = format!("{}{}{}", Self::helper(n % 1000), thousands[i], res);
            }
            n /= 1000;
            i += 1;
        }

        res.trim().to_string()
    }

    fn helper(num: i32) -> String {
        let less_than_20 = [
            "",
            " One",
            " Two",
            " Three",
            " Four",
            " Five",
            " Six",
            " Seven",
            " Eight",
            " Nine",
            " Ten",
            " Eleven",
            " Twelve",
            " Thirteen",
            " Fourteen",
            " Fifteen",
            " Sixteen",
            " Seventeen",
            " Eighteen",
            " Nineteen",
        ];
        let tens = [
            "", " Ten", " Twenty", " Thirty", " Forty", " Fifty", " Sixty", " Seventy", " Eighty",
            " Ninety",
        ];

        if num == 0 {
            "".to_string()
        } else if num < 20 {
            less_than_20[num as usize].to_string()
        } else if num < 100 {
            format!("{}{}", tens[(num / 10) as usize], Self::helper(num % 10))
        } else {
            format!(
                "{} Hundred{}",
                less_than_20[(num / 100) as usize],
                Self::helper(num % 100)
            )
        }
    }
}
```

La funcion externa maneja el caso especial del cero de entrada -- es la unica entrada que debe producir la palabra "Zero". El arreglo `thousands` mapea el indice del grupo a la palabra de escala: el indice 0 es el grupo de unidades (sin sufijo), 1 es Thousand, 2 es Million, 3 es Billion. El bucle `while` extrae tres digitos a la vez desde la derecha. La guarda `n % 1000 != 0` asegura que nunca se produce salida para un grupo de ceros, lo cual de lo contrario insertaria una palabra de escala colgante como "Thousand" sin ningun numero antes. La funcion `helper` es un pequeno conversor recursivo para numeros del 0 al 999. Al incrustar un espacio inicial en cada palabra de los arreglos de busqueda, la concatenacion siempre es limpia -- `format!` simplemente pega los fragmentos, y un unico `trim()` al final elimina el espacio inicial sobrante. La profundidad de recursion nunca excede dos (centenas -> decenas -> unidades), asi que esto es efectivamente iterativo en costo.

## Conclusion

Integer to English Words es un problema que recompensa el pensamiento estructural por sobre el analisis de casos por fuerza bruta. La idea clave es que el ingles naturalmente agrupa los digitos de tres en tres, asi que la conversion se descompone en un bucle externo sobre grupos de miles y un handler recursivo interno para numeros del 0 al 999. La irregularidad de los numeros del 11 al 19 se maneja con una tabla de busqueda plana, las decenas con otra, y las centenas con una llamada recursiva. Los espacios iniciales incorporados en los arreglos de palabras eliminan la logica complicada de espaciado. El resultado es una solucion limpia y legible que maneja cada caso borde -- desde cero hasta dos mil millones -- sin un solo condicional ad-hoc.
