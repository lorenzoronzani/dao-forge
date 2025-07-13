import { Actor } from "@dfinity/agent";

export interface Parameter {
    name?: string,
    type: string,
    fields?: Parameter[]
}

export interface MethodSignature {
    name: string;
    parameters: Parameter[];
    returnType: string;
}

export class CanisterAnalyzerService {
    static extractMethods(actor: any): MethodSignature[] {
        const methods: MethodSignature[] = [];

        // Get all method names from the actor
        const methodNames = this.getMethodNames(actor);

        methodNames.forEach(methodName => {
            const signature = this.extractMethodSignature(actor, methodName);
            if (signature) {
                methods.push(signature);
            }
        });

        return methods;
    }

    private static getMethodNames(actor: any): string[] {
        return Object.getOwnPropertyNames(actor).filter(name =>
            typeof actor[name] === 'function' && !name.startsWith('_')
        );
    }

    private static extractMethodSignature(actor: any, methodName: string): MethodSignature | undefined {
        const method = Actor.interfaceOf(actor)._fields.find(field => field[0] === methodName);

        if (!method) {
            return undefined;
        }

        const args = method[1].argTypes;
        const result = method[1].retTypes;

        const parameters = args.map((arg: any) => {
            // Check if the argument is a record
            if (arg._fields !== undefined) {
                const fields = arg._fields.map((field: any) => ({
                    name: field[0],
                    type: this.candidTypeToTypeScript(field[1]),
                }));

                // Return a single object representing the record parameter
                return {
                    type: 'record',
                    fields: fields,
                };
            }

            // Handle standard, non-record parameters
            return { type: this.candidTypeToTypeScript(arg) };
        });
        const returnType = this.candidTypeToTypeScript(result);

        return {
            name: methodName,
            parameters,
            returnType: returnType
        };
    }

    private static candidTypeToTypeScript(type: any): string {
        switch (type.name) {
            case 'text':
                return 'string';
            case 'nat32':
                return 'number';
            case 'record':
                return type.name;
            case undefined:
                return type[0].name;
            default:
                return 'unknown';
        }
    }
}
export default CanisterAnalyzerService;