import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Plus, X } from "lucide-react";

type MembersInfoCardProps = {
    id: string;
    title: string;
    members: string[];
    onValueChange: (id: string, value: string[]) => void;
}

export const MembersInfoCard = ({ id, title, members, onValueChange }: MembersInfoCardProps) => {
    const addMember = () => {
        onValueChange(id, [...members, '']);
    };

    const updateMember = (index: number, value: string) => {
        const updatedMembers = [...members];
        updatedMembers[index] = value;
        onValueChange(id, updatedMembers);
    };

    const removeMember = (index: number) => {
        const updatedMembers = [...members];
        updatedMembers.splice(index, 1);
        onValueChange(id, updatedMembers);
    };

    return (
        <Card>
            <CardContent className="pt-6">
                <div className="flex justify-between items-center mb-4">
                    <h3 className="text-lg font-medium">{title}</h3>
                    <Button
                        type="button"
                        variant="outline"
                        size="sm"
                        onClick={addMember}
                        className="flex items-center gap-1"
                    >
                        <Plus className="h-4 w-4" /> Add
                    </Button>
                </div>

                <div className="space-y-2">
                    {members.map((member, index) => (
                        <div key={`member-${index}`} className="flex gap-2">
                            <Input
                                value={member}
                                onChange={(e) => updateMember(index, e.target.value)}
                                placeholder="Principal ID"
                                className="flex-1"
                            />
                            <Button
                                type="button"
                                variant="ghost"
                                size="icon"
                                onClick={() => removeMember(index)}
                            >
                                <X className="h-4 w-4" />
                            </Button>
                        </div>
                    ))}
                </div>
            </CardContent>
        </Card>
    );
}