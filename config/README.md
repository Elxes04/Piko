# Configuration File README

# Piko Configuration Guide

This README provides information on how to customize the configuration file for the `Piko` application. The configuration file allows you to define how system information is displayed, enabling you to tailor the output to your preferences.

## Configuration File Location

The default configuration file is located at:

```
Piko/config/default_config.toml
```

You can create a custom configuration file by copying `default_config.toml` and modifying it as needed.

## Available Options

The configuration file contains several options that you can customize:

- **layout**: Defines the layout of the output. You can specify how you want the information to be presented. Options may include:
  - `compact`: A concise view of the system information.
  - `detailed`: A more verbose output that includes additional metrics.

- **show_os**: A boolean option to display the operating system information. Set to `true` to include OS details in the output.

- **show_cpu**: A boolean option to display CPU information. Set to `true` to include CPU details in the output.

- **show_memory**: A boolean option to display memory usage. Set to `true` to include memory metrics in the output.

- **custom_format**: A string that allows you to define a custom format for the output. You can use placeholders for different pieces of information (e.g., `{os}`, `{cpu}`, `{memory}`).

## Example Configuration

Here is an example of what the `default_config.toml` file might look like:

```toml
layout = "detailed"
show_os = true
show_cpu = true
show_memory = true
custom_format = "{os}\n{cpu}\n{memory}"
```

## Modifying the Configuration

To modify the configuration:

1. Open the `default_config.toml` file in a text editor.
2. Change the values of the options as desired.
3. Save the file and run the application to see the changes in the output.

## Conclusion

Customizing the configuration file allows you to control how system information is displayed by the `Piko` application. Adjust the settings to fit your needs and enjoy a personalized experience!
