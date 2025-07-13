import React from 'react';
import { Settings, Code, AlertTriangle } from 'lucide-react';
import { Badge } from "@/components/ui/badge";
import { Action } from '@/models/entities/Action';

interface ActionSectionProps {
    action: Action;
}

export const DisplayedActionSection: React.FC<ActionSectionProps> = ({ action }) => {
    const parameters = action.args;

    return (
        <div className="space-y-3">
            <div className="flex items-center gap-2">
                <Settings className="h-4 w-4 text-slate-500" />
                <h4 className="font-medium">Action to Execute</h4>
            </div>

            <div className="bg-slate-50 rounded-lg p-4 space-y-3">
                {/* Canister Information */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
                    <div>
                        <span className="text-slate-500">Canister ID:</span>
                        <p className="font-mono text-xs mt-1 break-all">{action.canisterId.toText()}</p>
                    </div>
                    <div>
                        <span className="text-slate-500">Method:</span>
                        <div className="flex items-center gap-2 mt-1">
                            <Badge variant="outline">{action.method}</Badge>
                        </div>
                    </div>
                </div>

                {/* Parameters */}
                {parameters.length > 0 && (
                    <div>
                        <span className="text-slate-500 text-sm">Parameters:</span>
                        <div className="mt-1 space-y-1">
                            {parameters.map((param, index) => (
                                <div key={index} className="bg-white p-2 rounded border text-xs font-mono break-words">
                                    <span className="text-slate-500">[{index}]</span> {param.replace(/,/g, ',\u200B')}
                                </div>
                            ))}
                        </div>
                    </div>
                )}

                {/* Method Signature Display */}
                <div className="pt-2 border-t">
                    <div className="flex items-center gap-1 mb-1">
                        <Code className="h-3 w-3 text-slate-500" />
                        <span className="text-xs text-slate-500">Call Preview:</span>
                    </div>
                    <code className="text-xs bg-white p-2 rounded border block break-words">
                        {`${action.method}(${parameters.join(', ')})`.replace(/,/g, ',\u200B')}
                    </code>
                </div>

                {/* Execution Info */}
                <div className="flex items-start gap-2 p-2 bg-blue-50 rounded text-xs">
                    <AlertTriangle className="h-3 w-3 text-blue-600 mt-0.5 flex-shrink-0" />
                    <span className="text-blue-800">
                        This action will be automatically executed if the voting passes and meets the quorum and approval threshold requirements.
                    </span>
                </div>
            </div>
        </div>
    );
};