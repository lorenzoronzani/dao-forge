import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { Users } from "lucide-react";
import { Dao } from "@/models/entities/Dao";
import { Principal } from "@dfinity/principal";
import { useState } from "react";
import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

enum VoterGroup {
    Board = 'board',
    Members = 'members',
    Both = 'both'
}

type VotersCardProps = {
    dao: Dao;
    onValueChange: (field: string, value: Principal[]) => void;
}

export const VotersCard = ({ dao, onValueChange }: VotersCardProps) => {
    const [voterGroup, setVoterGroup] = useState<VoterGroup | undefined>();

    const handleVoterGroupChange = (value: string) => {
        setVoterGroup(value as VoterGroup);
        onValueChange('votersWhitelist', getVoters(value as VoterGroup));
    };

    const getVoters = (voterGroup: VoterGroup | undefined): Principal[] => {
        if (!voterGroup) {
            return [];
        }
        switch (voterGroup) {
            case VoterGroup.Board:
                return dao.board;
            case VoterGroup.Members:
                return dao.members;
            case VoterGroup.Both:
                return [...new Set([...dao.board, ...dao.members])];
        }
    }

    return (
        <Card>
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <Users className="h-5 w-5" />
                    Voters
                </CardTitle>
                <CardDescription>Select who can participate in this voting</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
                <VerticalLabeledComponent label="Voters" htmlFor="votersWhitelist">
                    <Select
                        onValueChange={handleVoterGroupChange}
                        required
                    >
                        <SelectTrigger>
                            <SelectValue placeholder="Select Voters" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value={VoterGroup.Board}>Board Members Only ({dao.board.length})</SelectItem>
                            <SelectItem value={VoterGroup.Members}>Members Only ({dao.members.length})</SelectItem>
                            <SelectItem value={VoterGroup.Both}>Both Board and Members ({[...new Set([...dao.board, ...dao.members])].length})</SelectItem>
                        </SelectContent>
                    </Select>
                </VerticalLabeledComponent>

                <div className="mt-4 border rounded-md p-3 max-h-40 overflow-y-auto bg-slate-50">
                    <h4 className="text-sm font-medium mb-2">Selected Voters ({getVoters(voterGroup).length})</h4>
                    <div className="space-y-1">
                        {getVoters(voterGroup).map((voter, index) => (
                            <div key={index} className="text-xs font-mono text-slate-700 truncate">
                                {voter.toText()}
                            </div>
                        ))}
                        {getVoters(voterGroup).length === 0 && (
                            <p className="text-xs text-slate-500">No voters selected</p>
                        )}
                    </div>
                </div>
            </CardContent>
        </Card>
    )
}

export default VotersCard;