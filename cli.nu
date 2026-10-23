#! /usr/bin/env nu

const root_dir = path self .

def main [] {

}

def "main add" [
    name: string,
    description: string = ""
] {
    let name: string = $name | snake-case $in;
    let output_path = $root_dir | path join $"src/($name).rs"

    if ($output_path | path exists) {
        print $"Exercise (name) already exist"
        exit 1
    } else {
        let content = build-rust-code $name $description
        $content | save $output_path
        print $"src/($name).rs created..."
        append-module $name
        print $"Module '($name)' added to lib.rs file"
    }
}

def build-rust-code [
    exercise_name: string,
    description: string
]  {
    let template = open ($root_dir | path join "assets/base.rs")
    let context = { name: $exercise_name, description: $description }

    $template
        | str replace --all "{{name}}" $exercise_name
        | str replace --all "{{description}}" $description
}

def append-module [
    module: string
] {
    let lib_file = $root_dir | path join "src/lib.rs"
    $"mod ($module);" | save -a $lib_file
}

def snake-case [
    input: string
] {
    $input
    | str replace --all --regex '([a-z0-9])([A-Z])' '$1_$2'
    | str replace --all --regex '[\s\-]+' '_'
    | str lowercase
}