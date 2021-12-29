param(
[string]$a
)

Write-Host $a

$fnName = $a.split("(")[0]

$content = @"
pub fn $a` {
    todo!()
}

#[cfg(test)]
mod $fnName`_tests {
    use super::*;

    #[test]
    fn $fnName`_test_one() {
        todo!();
        assert_eq!($fnName`(), 1);
    }
}
"@

Add-Content "./src/lib.rs" "pub mod $fnName`;"

New-Item -Path './src' -Name "$fnName`.rs" -ItemType "file" -Value $content