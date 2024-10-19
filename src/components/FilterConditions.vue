<script setup lang="ts">
import { Condition } from '~/types/app-types'

const props = defineProps<{
    modelValue: Condition
    dtype: string
}>()

const emit = defineEmits(['update:modelValue'])

const selectCondition = ref<Condition | null>(null)

onMounted(() => {
    if (props.dtype === 'Boolean') {
        selectCondition.value = Condition.eq
        emit('update:modelValue', Condition.eq)
        return
    }
    if (props.dtype === 'String') {
        selectCondition.value = Condition.equals
        emit('update:modelValue', Condition.equals)
        return
    }
})

watch(
    () => props.modelValue,
    (newValue) => {
        selectCondition.value = newValue
    }
)

function updateCondition(condition: Condition) {
    if (condition === selectCondition.value) {
        selectCondition.value = null
        emit('update:modelValue', null)
    } else {
        selectCondition.value = condition
        emit('update:modelValue', condition)
    }
}

const fixedCondition = computed(() => {
    return ['Boolean', 'String'].includes(props.dtype)
})
</script>

<template>
    <div v-if="fixedCondition"></div>
    <SelectButton
        v-else
        :modelValue="selectCondition"
        :options="[
            Condition.lt,
            Condition.le,
            Condition.eq,
            Condition.neq,
            Condition.ge,
            Condition.gt,
            Condition.between,
        ]"
        @update:modelValue="updateCondition"
    >
        <template #option="slotProps">
            <IconLt v-if="slotProps.option === Condition.lt" class="h-4 w-4" />
            <IconLe v-if="slotProps.option === Condition.le" class="h-4 w-4" />
            <IconEq v-if="slotProps.option === Condition.eq" class="h-4 w-4" />
            <IconNeq
                v-if="slotProps.option === Condition.neq"
                class="h-4 w-4"
            />
            <IconGe v-if="slotProps.option === Condition.ge" class="h-4 w-4" />
            <IconGt v-if="slotProps.option === Condition.gt" class="h-4 w-4" />
            <IconBetween
                v-if="slotProps.option === Condition.between"
                class="h-4 w-4"
            />
        </template>
    </SelectButton>
</template>
