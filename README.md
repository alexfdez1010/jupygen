# Jupygen: Automatic Jupyter Notebook Generation with GPT-3.5

Jupygen, utilizing GPT-3.5, is your tool for rapidly learning new subjects or preparing educational Jupyter Notebooks. It generates notebooks based on your specific needs.

Currently, the binaries are not available. Thus, you will need to compile the code to generate them. Refer to [Development](#development) for that aim.

## Table of Contents

- [Requirements](#requirements)
- [Execution](#execution)
- [Development](#development)
- [Contribution Guidelines](#contribution-guidelines)
- [License](#license)

## Requirements

For successful operation of Jupygen, you'll require:

1. **Pandoc**: This is an open-source document converter which is crucial for transforming generated Markdown files into Jupyter Notebooks. To install pandoc, visit the [official website](https://pandoc.org/installing.html).
2. **OpenAI API Key**: This will have to be configured as an environment variable.

For acquiring an OpenAI API key, follow the steps:

1. Head to the [OpenAI website](https://www.openai.com/).
2. If you lack an account, click "Sign up" to create one.
3. Post login, make your way to the [API key management page](https://platform.openai.com/account/api-keys).
4. Click on "Create new secret key" and record the generated API key (since you won't see this key again).

After obtaining the API key, set an environment variable named `OPENAI_TOKEN` with the API key you got from OpenAI. The process for setting environment variables depends on your operating system.

## Execution

To execute the app, simply run the following command at the location where you've downloaded the binary:

```bash
./jupygen
```
For Mac OS X and Linux users, you might have to provide execution permissions to the file:

```bash
chmod +x jupygen
```
Windows users can execute the app simply by double-clicking the executable file.

Post this, a GUI will appear where you can specify the description of the notebook you wish to generate, along with the location where you desire to save it.

## Development

To contribute to the development of the app, you'll require Tauri and its related dependencies. Comprehensive information can be found on the official [Tauri website](https://tauri.studio/en/docs/getting-started/intro).

## Contribution Guidelines

We appreciate contributions aimed at improving Jupygen. If you wish to contribute, please fork the repository and create a pull request with your proposed changes. We will review and merge the changes if deemed fit.

## License

This project operates under the MIT License. Please refer to the [LICENSE](LICENSE) file for more information.
